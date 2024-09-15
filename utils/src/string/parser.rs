use crate::string::ast::{PegChar, PegString};
use crate::string::register::{Base, CacheResult, CacheType, Text};
use daybreak::Parser;

impl Base for Parser<'_, CacheType, CacheResult> {
    type CT = CacheType;
    type CR = CacheResult;
}

impl Text for Parser<'_, CacheType, CacheResult> {
    #[daybreak::strict]
    #[daybreak::memoize(PegString)]
    fn peg_string(&mut self) -> Option<PegString> {
        let pos = self.stream.mark();
        let mut cut = false;
        if let Some(result) = || -> Option<PegString> {
            self.expect("r")?;
            self.stream.strict(true);
            cut = true;
            self.expect("\"")?;
            let mut string = String::new();
            while let Some(ch) = self.scan(|c| c != '"') {
                string.push(ch)
            }
            self.expect("\"")?;
            Some(PegString::Raw(string))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        self.stream.strict(false);
        if let Some(result) = || -> Option<PegString> {
            self.expect("\"")?;
            self.stream.strict(true);
            let mut string = Vec::new();
            while self.lookahead(|c| c != '"').is_some() {
                let ch = self.peg_char()?;
                string.push(ch)
            }
            self.expect("\"")?;
            Some(PegString::Plain(string))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[daybreak::memoize(PegChar)]
    fn peg_char(&mut self) -> Option<PegChar> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<PegChar> {
            let ch = self.scan(|c| c != '\\')?;
            Some(PegChar::Plain(ch))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<PegChar> {
            self.expect("\\\\")?;
            Some(PegChar::Backlash)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<PegChar> {
            self.expect("\\\"")?;
            Some(PegChar::Quotation)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<PegChar> {
            self.expect("\\n")?;
            Some(PegChar::Newline)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<PegChar> {
            self.expect("\\r")?;
            Some(PegChar::Return)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<PegChar> {
            self.expect("\\t")?;
            Some(PegChar::Tab)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }
}
