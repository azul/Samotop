use std::io;
use std::io::Write;
use model::response::SmtpReply;

static SERIALIZER: SmtpSerializer = SmtpSerializer;

type Result = io::Result<()>;

pub trait SmtpAnswerSerializer {
    fn write(&self, buf: &mut Write, answer: SmtpReply) -> Result;
}

pub struct SmtpSerializer;

impl SmtpSerializer {
    pub fn answer_serializer<'a>() -> &'a SmtpAnswerSerializer {
        &SERIALIZER
    }

    fn write_reply(&self, mut buf: &mut Write, reply: SmtpReply) -> Result {
        match reply {
            SmtpReply::None => Ok(()),
            _ => {
                let code = reply.code();
                let text = reply.text();
                let items = reply.items();

                if items.is_empty() {
                    try!(self.write_reply_end(&mut buf, code, &text));
                } else {
                    try!(self.write_reply_continued(&mut buf, code, &text));
                    for i in 0..items.len() {
                        if i == items.len() - 1 {
                            try!(self.write_reply_end(&mut buf, code, &items[i]));
                        } else {
                            try!(self.write_reply_continued(&mut buf, code, &items[i]));
                        }
                    }
                }
                buf.write_all(b"\r\n")
            }
        }
    }

    fn write_reply_end(&self, buf: &mut Write, code: u16, text: &str) -> Result {
        write!(buf, "{} {}", code, text)
    }
    fn write_reply_continued(&self, buf: &mut Write, code: u16, text: &str) -> Result {
        write!(buf, "{}-{}", code, text)
    }
}

impl SmtpAnswerSerializer for SmtpSerializer {
    fn write(&self, mut buf: &mut Write, reply: SmtpReply) -> Result {
        self.write_reply(&mut buf, reply)
    }
}
