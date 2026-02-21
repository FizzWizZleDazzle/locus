//! Email service using Resend

use crate::AppError;
use resend_rs::Resend;
use resend_rs::types::CreateEmailBaseOptions;

#[derive(Clone)]
pub struct EmailService {
    client: Resend,
    from_email: String,
    from_name: String,
    frontend_base_url: String,
}

impl EmailService {
    /// Create a new email service
    pub fn new(
        api_key: String,
        from_email: String,
        from_name: String,
        frontend_base_url: String,
    ) -> Self {
        let client = Resend::new(&api_key);
        Self {
            client,
            from_email,
            from_name,
            frontend_base_url,
        }
    }

    /// Send verification email to user
    pub async fn send_verification_email(
        &self,
        to_email: &str,
        to_name: &str,
        verification_token: &str,
    ) -> Result<(), AppError> {
        let verification_link = format!(
            "{}/verify-email?token={}",
            self.frontend_base_url.trim_end_matches('/'),
            verification_token
        );

        let html_body = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Verify Your Email</title>
</head>
<body style="margin: 0; padding: 0; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif; background-color: #f5f5f5;">
    <table role="presentation" width="100%" cellspacing="0" cellpadding="0" border="0">
        <tr>
            <td align="center" style="padding: 40px 20px;">
                <table role="presentation" width="600" cellspacing="0" cellpadding="0" border="0" style="background-color: #ffffff; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                    <!-- Header -->
                    <tr>
                        <td style="padding: 40px 40px 20px 40px; text-align: center;">
                            <h1 style="margin: 0; font-size: 28px; font-weight: 600; color: #1a1a1a;">Verify Your Email</h1>
                        </td>
                    </tr>

                    <!-- Body -->
                    <tr>
                        <td style="padding: 20px 40px 40px 40px;">
                            <p style="margin: 0 0 20px 0; font-size: 16px; line-height: 1.5; color: #4a4a4a;">
                                Hi {},
                            </p>
                            <p style="margin: 0 0 30px 0; font-size: 16px; line-height: 1.5; color: #4a4a4a;">
                                Thanks for signing up for Locus! To get started, please verify your email address by clicking the button below:
                            </p>

                            <!-- Button -->
                            <table role="presentation" width="100%" cellspacing="0" cellpadding="0" border="0">
                                <tr>
                                    <td align="center" style="padding: 10px 0 30px 0;">
                                        <a href="{}" style="display: inline-block; padding: 14px 32px; background-color: #2563eb; color: #ffffff; text-decoration: none; border-radius: 6px; font-size: 16px; font-weight: 500;">Verify Email Address</a>
                                    </td>
                                </tr>
                            </table>

                            <p style="margin: 0 0 10px 0; font-size: 14px; line-height: 1.5; color: #6b6b6b;">
                                Or copy and paste this link into your browser:
                            </p>
                            <p style="margin: 0 0 30px 0; font-size: 14px; line-height: 1.5; color: #2563eb; word-break: break-all;">
                                {}
                            </p>

                            <p style="margin: 0; font-size: 14px; line-height: 1.5; color: #6b6b6b;">
                                This link will expire in <strong>1 hour</strong>.
                            </p>
                        </td>
                    </tr>

                    <!-- Footer -->
                    <tr>
                        <td style="padding: 30px 40px 40px 40px; border-top: 1px solid #e5e5e5;">
                            <p style="margin: 0; font-size: 12px; line-height: 1.5; color: #9b9b9b;">
                                If you didn't create an account with Locus, you can safely ignore this email.
                            </p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>"#,
            to_name, verification_link, verification_link
        );

        let text_body = format!(
            r#"Hi {},

Thanks for signing up for Locus! To get started, please verify your email address by clicking the link below:

{}

This link will expire in 1 hour.

If you didn't create an account with Locus, you can safely ignore this email.

---
Locus Team"#,
            to_name, verification_link
        );

        let from = format!("{} <{}>", self.from_name, self.from_email);

        let email = CreateEmailBaseOptions::new(&from, [to_email], "Verify your email for Locus")
            .with_html(&html_body)
            .with_text(&text_body);

        self.client
            .emails
            .send(email)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to send email: {}", e)))?;

        Ok(())
    }

    /// Send password reset email to user
    pub async fn send_password_reset_email(
        &self,
        to_email: &str,
        to_name: &str,
        reset_token: &str,
    ) -> Result<(), AppError> {
        let reset_link = format!(
            "{}/reset-password?token={}",
            self.frontend_base_url.trim_end_matches('/'),
            reset_token
        );

        let html_body = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Reset Your Password</title>
</head>
<body style="margin: 0; padding: 0; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif; background-color: #f5f5f5;">
    <table role="presentation" width="100%" cellspacing="0" cellpadding="0" border="0">
        <tr>
            <td align="center" style="padding: 40px 20px;">
                <table role="presentation" width="600" cellspacing="0" cellpadding="0" border="0" style="background-color: #ffffff; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                    <!-- Header -->
                    <tr>
                        <td style="padding: 40px 40px 20px 40px; text-align: center;">
                            <h1 style="margin: 0; font-size: 28px; font-weight: 600; color: #1a1a1a;">Reset Your Password</h1>
                        </td>
                    </tr>

                    <!-- Body -->
                    <tr>
                        <td style="padding: 20px 40px 40px 40px;">
                            <p style="margin: 0 0 20px 0; font-size: 16px; line-height: 1.5; color: #4a4a4a;">
                                Hi {},
                            </p>
                            <p style="margin: 0 0 30px 0; font-size: 16px; line-height: 1.5; color: #4a4a4a;">
                                We received a request to reset your password for your Locus account. Click the button below to create a new password:
                            </p>

                            <!-- Button -->
                            <table role="presentation" width="100%" cellspacing="0" cellpadding="0" border="0">
                                <tr>
                                    <td align="center" style="padding: 10px 0 30px 0;">
                                        <a href="{}" style="display: inline-block; padding: 14px 32px; background-color: #2563eb; color: #ffffff; text-decoration: none; border-radius: 6px; font-size: 16px; font-weight: 500;">Reset Password</a>
                                    </td>
                                </tr>
                            </table>

                            <p style="margin: 0 0 10px 0; font-size: 14px; line-height: 1.5; color: #6b6b6b;">
                                Or copy and paste this link into your browser:
                            </p>
                            <p style="margin: 0 0 30px 0; font-size: 14px; line-height: 1.5; color: #2563eb; word-break: break-all;">
                                {}
                            </p>

                            <p style="margin: 0 0 20px 0; font-size: 14px; line-height: 1.5; color: #6b6b6b;">
                                This link will expire in <strong>30 minutes</strong>.
                            </p>

                            <p style="margin: 0; font-size: 14px; line-height: 1.5; color: #dc2626; background-color: #fef2f2; padding: 12px; border-radius: 4px; border-left: 3px solid #dc2626;">
                                <strong>Security notice:</strong> If you didn't request a password reset, you can safely ignore this email. Your password will remain unchanged.
                            </p>
                        </td>
                    </tr>

                    <!-- Footer -->
                    <tr>
                        <td style="padding: 30px 40px 40px 40px; border-top: 1px solid #e5e5e5;">
                            <p style="margin: 0; font-size: 12px; line-height: 1.5; color: #9b9b9b;">
                                This password reset was requested from your Locus account. If you believe this request is suspicious, please contact support.
                            </p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>"#,
            to_name, reset_link, reset_link
        );

        let text_body = format!(
            r#"Hi {},

We received a request to reset your password for your Locus account. Click the link below to create a new password:

{}

This link will expire in 30 minutes.

SECURITY NOTICE: If you didn't request a password reset, you can safely ignore this email. Your password will remain unchanged.

---
Locus Team"#,
            to_name, reset_link
        );

        let from = format!("{} <{}>", self.from_name, self.from_email);

        let email = CreateEmailBaseOptions::new(&from, [to_email], "Reset your Locus password")
            .with_html(&html_body)
            .with_text(&text_body);

        self.client
            .emails
            .send(email)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to send email: {}", e)))?;

        Ok(())
    }
}
