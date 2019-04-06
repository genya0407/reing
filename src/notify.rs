use lettre::{SmtpTransport, SmtpClient, Transport};
use lettre_email::EmailBuilder;
use lettre::smtp::authentication::{Mechanism, Credentials};
use model;
use std::thread;
use std::env;

pub fn send_email(question: model::Question) {
    let builder = thread::Builder::new().name("send-email-thread".into());
    builder.spawn(move || {
        let question_url = format!(
            "https://{}/admin/question/{}",
            env::var("APPLICATION_DOMAIN").expect("APPLICATION_DOMAIN not specified"),
            question.id
        );
        let email = EmailBuilder::new()
            .to(env::var("ADMIN_EMAIL").expect("ADMIN_EMAIL not specified"))
            .from(env::var("MAILER_FROM").expect("MAILER_FROM not specified"))
            .subject("質問が投稿されました")
            .html(
                format!(
                    "<p>質問が投稿されました</p><p>{}</p><p><a href='{}'>{}</a>",
                    htmlescape::encode_minimal(&question.body), question_url, question_url
                )
            )
            .build()
            .expect("Failed to build email.");

        let domain = env::var("MAILER_DOMAIN").expect("MAILER_DOMAIN not specified");
        let builder = SmtpClient::new_simple(&domain).expect("Failed to initialize builder")
                        .authentication_mechanism(Mechanism::Plain)
                        .credentials(
                            Credentials::new(
                                env::var("MAILER_USERNAME").expect("MAILER_USERNAME not specified"),
                                env::var("MAILER_PASSWORD").expect("MAILER_PASSWORD not specified")
                            )
                        );
        let mut mailer = SmtpTransport::new(builder);
        mailer.send(email.into()).expect("Failed to send email.");
    }).unwrap();
}
