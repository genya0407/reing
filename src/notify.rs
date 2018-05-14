use lettre::{EmailTransport, SmtpTransport};
use lettre_email::EmailBuilder;
use lettre::smtp::authentication::{Mechanism, Credentials};
use model;
use std::thread;
use std::env;

pub fn send_email(question: model::Question) {
    thread::spawn(move || {
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
                    "<p>質問が投稿されました</p><p><a href='{}'>{}</a>",
                    question_url, question_url
                )
            )
            .build()
            .expect("Failed to build email.");

        let domain = env::var("MAILER_DOMAIN").expect("MAILER_DOMAIN not specified");
        let mut mailer = SmtpTransport::simple_builder(&domain).expect("Failed to initialize builder")
                            .authentication_mechanism(Mechanism::Plain)
                            .credentials(
                                Credentials::new(
                                    env::var("MAILER_USERNAME").expect("MAILER_USERNAME not specified"),
                                    env::var("MAILER_PASSWORD").expect("MAILER_PASSWORD not specified")
                                )
                            )
                            .build();
        mailer.send(&email).expect("Failed to send email.");
    });
}