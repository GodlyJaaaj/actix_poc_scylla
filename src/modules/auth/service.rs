use crate::config::Config;
use crate::models::User;
use crate::modules::auth::dto::{LoginQuery, RegisterQuery};
use crate::modules::auth::repository::AuthRepository;
use actix_identity::Identity;
use actix_web::{web, HttpMessage, HttpRequest};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel::PgConnection;
use serde::Serialize;
use std::error::Error;
use uuid::Uuid;

#[derive(Serialize)]
pub struct TokenResponse {
    access_token: String,
    refresh_token: String,
}

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

        println!("{}", config.smtp.email_from);

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
                Ce lien expirera dans 24 heures.\n\n\
                Cordialement,\n\
                L'équipe",
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

    pub fn verify(conn: &mut PgConnection, token: &str) -> Result<(), Box<dyn Error>> {
        // Récupérer le token de vérification
        let token = match AuthRepository::find_verification_token(conn, token)? {
            Some(token) => token,
            None => return Err("Token not found".into()),
        };

        // Vérifier si le token a expiré ou a déjà été utilisé
        if token.expires_at < Utc::now() {
            return Err("Token has expired".into());
        }

        if token.used_at.is_some() {
            return Err("Token has already been used".into());
        }

        // Marquer le token comme utilisé
        AuthRepository::use_verification_token(conn, &token)?;

        Ok(())
    }

    pub fn forgot_password(conn: &mut PgConnection, email: &str) -> Result<(), Box<dyn Error>> {
        // Vérifier si l'utilisateur existe
        let user = match AuthRepository::find_user_by_email(conn, email)? {
            Some(user) => user,
            None => return Err("User not found".into()),
        };

        // TODO: Générer un token de réinitialisation et l'enregistrer
        // TODO: Envoyer un email avec le lien de réinitialisation

        // Pour l'instant, on simule que tout s'est bien passé
        Ok(())
    }

    pub fn reset_password(
        conn: &mut PgConnection,
        token: &str,
        new_password: &str,
    ) -> Result<(), Box<dyn Error>> {
        // TODO: Vérifier que le token est valide et récent
        // TODO: Récupérer l'utilisateur associé au token
        // TODO: Mettre à jour le mot de passe haché

        // Simulation d'un hash de mot de passe
        let _hashed_password = crate::utils::password::hash_password(new_password)?;

        // Simule une réinitialisation réussie
        Ok(())
    }
}
