use base64::Engine;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Display};

#[derive(Debug, Serialize, Deserialize)]
pub enum CustomId<'a> {
    ButtonClowcardInfo(Cow<'a, str>),
}

impl Display for CustomId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct FormatterWrapper<'a: 'b, 'b>(&'b mut std::fmt::Formatter<'a>);
        impl base64::write::StrConsumer for FormatterWrapper<'_, '_> {
            fn consume(&mut self, buf: &str) {
                self.0.write_str(buf).unwrap()
            }
        }

        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, write::EncoderStringWriter};
        use bincode::{config, serde::encode_into_std_write};

        let f = FormatterWrapper(f);
        let mut bw64 = EncoderStringWriter::from_consumer(f, &URL_SAFE_NO_PAD);
        encode_into_std_write(self, &mut bw64, config::standard()).unwrap();
        Ok(())
    }
}

impl<'a> From<&'a str> for CustomId<'a> {
    fn from(value: &'a str) -> Self {
        fn try_from(value: &'_ str) -> Result<CustomId<'_>, anyhow::Error> {
            use base64::engine::general_purpose::URL_SAFE_NO_PAD;
            use bincode::{config, serde::decode_from_slice};
            let bytes = URL_SAFE_NO_PAD.decode(value)?;
            Ok(decode_from_slice(&bytes, config::standard())?.0)
        }
        try_from(value).expect("CustomId should be valid")
    }
}
