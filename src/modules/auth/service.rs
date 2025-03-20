use crate::config::Config;
use crate::models::User;
use crate::modules::auth::dto::{LoginQuery, RegisterQuery, ResetPasswordQuery, VerifyQuery};
use crate::modules::auth::repository::AuthRepository;
use actix_identity::Identity;
use actix_web::{web, HttpMessage, HttpRequest};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel::PgConnection;
use std::error::Error;
use uuid::Uuid;

pub struct AuthService;

impl AuthService {
    pub fn register(
        conn: &mut PgConnection,
        user_data: &RegisterQuery,
    ) -> Result<User, Box<dyn Error>> {
        // Check if user already exists
        if let Some(_) = AuthRepository::find_user_by_email(conn, &user_data.email)? {
            return Err("User with this email already exists".into());
        }

        // Create user and associated account
        let user = AuthRepository::create_user_account(conn, user_data)?;

        Ok(user)
    }

    pub fn login(
        req: &HttpRequest,
        conn: &mut PgConnection,
        login_data: &LoginQuery,
    ) -> Result<User, Box<dyn Error>> {
        use crate::models::Account;
        use crate::schema::accounts::dsl::*;
        use crate::utils::password::verify_password;

        // Find user by email
        let user = match AuthRepository::find_user_by_email(conn, &login_data.email)? {
            Some(user) => user,
            None => return Err("User not found".into()),
        };

        // Find the associated account with credentials
        let account = accounts
            .filter(user_id.eq(user.id))
            .filter(account_type.eq("credentials"))
            .filter(deleted_at.is_null())
            .first::<Account>(conn)
            .optional()?
            .ok_or("No credentials account found")?;

        // Get the stored password hash and verify
        let stored_hash = account.password.ok_or("No password set for this account")?;
        if !verify_password(&login_data.password, &stored_hash)? {
            return Err("Invalid password".into());
        }

        // Password verified, log the user in
        let _ = Identity::login(&req.extensions(), user.id.to_string());

        Ok(user)
    }

    pub fn logout(id: Identity) -> Result<(), Box<dyn Error>> {
        id.logout();
        Ok(())
    }

    pub fn request_verification(
        conn: &mut PgConnection,
        user_id: Uuid,
        config: &web::Data<Config>,
    ) -> Result<(), Box<dyn Error>> {
        use lettre::transport::smtp::authentication::Credentials;
        use lettre::transport::smtp::client::{Tls, TlsParameters};
        use lettre::{Message, SmtpTransport, Transport};
        use rand::{distributions::Alphanumeric, Rng};

        // Récupérer le user pour son mail
        let user = match AuthRepository::find_user_by_id(conn, user_id)
            .map_err(|e| format!("Failed to find user: {}", e))?
        {
            Some(user) => user,
            None => return Err("User not found".into()),
        };

        // Générer un token random
        let token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        let expiration = Utc::now() + Duration::minutes(10);
        AuthRepository::create_verification_token(conn, user_id, &token, expiration)?;

        let email = Message::builder()
            .from(
                config
                    .smtp
                    .email_from
                    .parse()
                    .map_err(|e| format!("Invalid from email: {}", e))?,
            )
            .to(user
                .email
                .parse()
                .map_err(|e| format!("Invalid recipient email: {}", e))?)
            .subject("Vérification de votre adresse email")
            .body(format!(
                "Bonjour {},\n\n\
                Veuillez cliquer sur le lien suivant pour vérifier votre adresse email :\n\
                {}/verify?token={}\n\n\
                Ce lien expirera dans 10 minutes.\n\n\
                L'équipe Scylla",
                user.name, config.smtp.frontend_url, token
            ))?;

        let creds = Credentials::new(config.smtp.username.clone(), config.smtp.password.clone());

        // Build different transports based on TLS mode
        let mailer = match config.smtp.tls_mode.to_lowercase().as_str() {
            "none" => SmtpTransport::builder_dangerous(&config.smtp.server)
                .credentials(creds)
                .port(config.smtp.port)
                .build(),
            "required" => {
                let tls_parameters = TlsParameters::new(config.smtp.server.clone())
                    .map_err(|e| format!("TLS error: {}", e))?;

                SmtpTransport::builder_dangerous(&config.smtp.server)
                    .credentials(creds)
                    .port(config.smtp.port)
                    .tls(Tls::Required(tls_parameters))
                    .build()
            }
            _ => {
                // "opportunistic" - default
                SmtpTransport::builder_dangerous(&config.smtp.server)
                    .credentials(creds)
                    .port(config.smtp.port)
                    .tls(Tls::Opportunistic(
                        TlsParameters::new(config.smtp.server.clone())
                            .map_err(|e| format!("TLS error: {}", e))?,
                    ))
                    .build()
            }
        };

        // Send the email with detailed error handling
        mailer.send(&email)?;

        Ok(())
    }

    pub fn verify(
        conn: &mut PgConnection,
        verify_data: &VerifyQuery,
    ) -> Result<(), Box<dyn Error>> {
        // retrieve the verification token
        let token = match AuthRepository::find_verification_token(conn, &verify_data.token)? {
            Some(token) => token,
            None => return Err("Token not found".into()),
        };

        // verify if the token has expired or has already been used
        if token.expires_at < Utc::now() {
            return Err("Token has expired".into());
        }

        if token.used_at.is_some() {
            return Err("Token has already been used".into());
        }

        // set the token as used
        AuthRepository::use_verification_token(conn, &token)?;

        Ok(())
    }

    pub fn forgot_password(
        conn: &mut PgConnection,
        user_id: Uuid,
        config: &web::Data<Config>,
    ) -> Result<(), Box<dyn Error>> {
        use lettre::transport::smtp::authentication::Credentials;
        use lettre::transport::smtp::client::{Tls, TlsParameters};
        use lettre::{Message, SmtpTransport, Transport};
        use rand::{distributions::Alphanumeric, Rng};

        // Récupérer le user pour son mail
        let user = match AuthRepository::find_user_by_id(conn, user_id)
            .map_err(|e| format!("Failed to find user: {}", e))?
        {
            Some(user) => user,
            None => return Err("User not found".into()),
        };

        // Générer un token random
        let token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        let expiration = Utc::now() + Duration::minutes(10);
        AuthRepository::create_reset_password_token(conn, user_id, &token, expiration)?;

        let email = Message::builder()
            .from(
                config
                    .smtp
                    .email_from
                    .parse()
                    .map_err(|e| format!("Invalid from email: {}", e))?,
            )
            .to(user
                .email
                .parse()
                .map_err(|e| format!("Invalid recipient email: {}", e))?)
            .subject("Réinitialisation de votre mot de passe")
            .body(format!(
                "Bonjour {},\n\n\
                Veuillez cliquer sur le lien suivant pour réinitialiser votre mot de passe :\n\
                {}/reset-password?token={}\n\n\
                Ce lien expirera dans 10 minutes.\n\n\
                L'équipe Scylla",
                user.name, config.smtp.frontend_url, token
            ))?;

        let creds = Credentials::new(config.smtp.username.clone(), config.smtp.password.clone());

        // Build different transports based on TLS mode
        let mailer = match config.smtp.tls_mode.to_lowercase().as_str() {
            "none" => SmtpTransport::builder_dangerous(&config.smtp.server)
                .credentials(creds)
                .port(config.smtp.port)
                .build(),
            "required" => {
                let tls_parameters = TlsParameters::new(config.smtp.server.clone())
                    .map_err(|e| format!("TLS error: {}", e))?;

                SmtpTransport::builder_dangerous(&config.smtp.server)
                    .credentials(creds)
                    .port(config.smtp.port)
                    .tls(Tls::Required(tls_parameters))
                    .build()
            }
            _ => {
                // "opportunistic" - default
                SmtpTransport::builder_dangerous(&config.smtp.server)
                    .credentials(creds)
                    .port(config.smtp.port)
                    .tls(Tls::Opportunistic(
                        TlsParameters::new(config.smtp.server.clone())
                            .map_err(|e| format!("TLS error: {}", e))?,
                    ))
                    .build()
            }
        };

        // Send the email with detailed error handling
        mailer.send(&email)?;

        Ok(())
    }

    pub fn reset_password(
        conn: &mut PgConnection,
        reset_data: &ResetPasswordQuery,
    ) -> Result<(), Box<dyn Error>> {
        // verify matching passwords
        if reset_data.password != reset_data.password_confirm {
            return Err("Passwords do not match".into());
        }

        // retrieve the reset password token
        let token = match AuthRepository::find_reset_password_token(conn, &reset_data.token)? {
            Some(token) => token,
            None => return Err("Token not found".into()),
        };

        // verify if the token has expired or has already been used
        if token.expires_at < Utc::now() {
            return Err("Token has expired".into());
        }

        if token.used_at.is_some() {
            return Err("Token has already been used".into());
        }

        // set the token as used
        AuthRepository::use_reset_password_token(conn, &token, &reset_data.password)?;

        Ok(())
    }
}
