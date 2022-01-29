use std::fmt;

pub enum WordEnum {
    // surface
    OnSurfaceable,
    // specifies days and dates
    OnDateable,
    // refers TV or other devices
    OnDeviceable,
    // refers the parts of the body
    OnPartsOfBody,
    // to refer a state
    OnStateable,
    // indicate a place
    AtPlace,
    // refer an email address
    AtEmail,
    // refer a time
    AtTime,
    // indicate ones' activity
    AtSomeoneActivity,
    // to indicate a location
    InLocation,
    // used while doing something
    InDoing,
    // indicate opinion
    InFeeling,
    // SDMSY aka: specify Day, Month, Season, Year
    InSDMSY,
    // to indicate color, shape and size
    InPhysicalPropertyable,
    // to indicate the direction, place
    ToDirectionPlaceable,
    // to indicate relationship
    ToRelationShipable,
    // to indicate a limit
    ToLimitable,
    // to refer a period
    ToPeriodable,
    // to indicate relating to, belonging to
    OfBelongable,
    // to indicate reference
    OfReferenceable,
    // to specify the number or an amount
    OfNumerable,
    // to indicate the reason or because of
    ForReason,
    // to indicate the duration or time
    ForTimeDuration,
    // specify the use of something
    ForSomething,
}

pub impl WordEnum {
    pub fn from(word: String) -> WordEnum {
        WordEnum
    }
    pub fn inspect_comments(word: Option<WordEnum>) -> &'static str {
        match word {
            WordEnum::OnSurfaceable => "refers a surface of something",
            WordEnum::OnDateable => "specifies days and dates",
            WordEnum::OnDeviceable => "refers TV or other devices",
            WordEnum::OnPartsOfBody => "refers the parts of the body",
            WordEnum::OnStateable => "to refer a state",
            WordEnum::AtPlace => "to refer a state",
            WordEnum::AtEmail => "to refer an email address",
            WordEnum::AtTime => "to refer a time",
            WordEnum::AtSomeoneActivity => "indicate ones' activity",
            WordEnum::InLocation => "to indicate a location",
            WordEnum::InDoing => "used while doing something",
            WordEnum::InFeeling => "to indicate opinion, belief, feeling, etc.",
            WordEnum::InSDMSY => "specify day, month, season, year",
            WordEnum::InPhysicalPropertyable => "to indicate color, shape and size",
            WordEnum::ToDirectionPlaceable => "to indicate the direction, place",
            WordEnum::ToRelationShipable => "to indicate relationship",
            WordEnum::ToLimitable => "to indicate a limit",
            WordEnum::ToPeriodable => "to refer a period",
            WordEnum::OfBelongable => "to indicate relating to, belonging to",
            WordEnum::OfReferenceable => "to indicate reference",
            WordEnum::OfNumerable => "to specify the number or an amount",
            WordEnum::ForReason => "to indicate the reason or because of",
            WordEnum::ForTimeDuration => "to indicate the duration or time",
            WordEnum::ForSomething => "specify the use of something",
        }
    }
}

impl fmt::Display for WordEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) {
        write!(f, "The word {} is {} \r\n \t it's means: {}", {
            // self.word, result, self.comments(self)
            let result = self.parse();
            (self.word, result, WordEnum::inspect_comments(result));
        })
    }
}
