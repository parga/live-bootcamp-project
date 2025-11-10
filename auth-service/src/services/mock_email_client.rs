use tracing::info;

use crate::domain::{email::Email, EmailClient};

#[derive(Default)]
pub struct MockEmailClient;

#[async_trait::async_trait]
impl EmailClient for MockEmailClient {
    async fn send_email(
        &self,
        recipient: &Email,
        subject: &str,
        content: &str,
    ) -> Result<(), String> {
        let current_span: tracing::Span = tracing::Span::current();
        current_span.in_scope(|| {
            info!(
                "Sending email to {} with subject: {} and content: {}",
                recipient.as_ref(),
                subject,
                content
            );
            Ok(())
        })
    }
}
