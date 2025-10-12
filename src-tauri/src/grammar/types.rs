#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConjugationPattern {
    Dictionary,
    MasuForm,
    Past,
    Negative,
    PastNegative,
    TeForm,
    TaiForm,
    TakunaiForm,
    TakattaForm,
    TaraConditional,
    BaConditional,
    TariForm,
    Potential,
    Passive,
    Causative,
    CausativePassive,
    Volitional,
    Imperative,
    TeIru,
    TeRequest,
    TeShimau,
    TeMiru,
    Nagara,
    Must,              // nakereba naranai / nakute wa ikenai
    TeKudasai,         // te kudasai - please do
    Node,              // node - because
    MaeNi,             // mae ni - before
    TeKara,            // te kara - after doing
    TaKotoGaAru,       // ta koto ga aru - have done before
    Mashou,            // mashou - let's/shall we
    TeMoIi,            // te mo ii - may/it's okay to
    TeWaIkenai,        // te wa ikenai - must not
    Sugiru,            // sugiru - too much
    HouGaIi,           // hou ga ii - had better
    NakuchaIkenai,     // nakucha ikenai - must (casual)
    NaideKudasai,      // naide kudasai - please don't
    MasenKa,           // masen ka - won't you/invitation
    MashouKa,          // mashou ka - shall I
    TsumoriDesu,       // tsumori desu - intention
    Deshou,            // deshou - probably
    MadaTeImasen,      // mada te imasen - not yet
    NDesu,             // n desu - explanatory
    TeAru,             // te aru - has been done
    TeKureru,          // te kureru - do for me
    TeAgeru,           // te ageru - do for someone
    TeOku,             // te oku - do in advance
    Yasui,             // yasui - easy to
    Nikui,             // nikui - difficult to
    TeMorau,           // te morau - have someone do
    TeSumimasen,       // te sumimasen - sorry for doing
    TeKureteArigatou,  // te kurete arigatou - thank you for
    TeYokatta,         // te yokatta - glad that
    TeMo,              // te mo - even if
    Naide,             // naide - without doing
    NakuteMoIi,        // nakute mo ii - don't have to
    BaYokatta,         // ba yokatta - should have
    Nasai,             // nasai - command form
    HazuDesu,          // hazu desu - should/expected to
    Tagaru,            // tagaru - show signs of wanting
    TeItadakemasenKa,  // te itadakemasen ka - could you please
    TaraDou,           // tara dou desu ka - how about
    ToIi,              // to ii desu - I hope
    GaHoshii,          // ga hoshii - want
    ShikaNai,          // shika nai - only/nothing but
    ToIu,              // to iu - called/named
    DictionaryTo,      // dictionary + to - when/if conditional
    Nara,              // nara - if it's the case that
    Shi,               // shi - and what's more
    KaDouKa,           // ka dou ka - whether or not
    KotoNiSuru,        // koto ni suru - decide to
    Noni,              // noni - despite/although
    KotoNiNaru,        // koto ni naru - it has been decided that
    NiIku,             // ni iku - go to do
    SouDesuAppearance, // sou desu - looks like (appearance)
    SouDesuHearsay,    // sou desu - I heard that (hearsay)
    KamoShirenai,      // kamo shirenai - might/maybe
    Mitai,             // mitai - looks like/seems
    // N3 patterns
    YouNiSuru,   // you ni suru - try to/make sure to
    Masaka,      // masaka - surely not (adverb)
    Hajimeru,    // hajimeru - begin to
    Rashii,      // rashii - seems like/typical
    Gurai,       // gurai/kurai - about/approximately
    NiYoruTo,    // ni yoru to - according to
    Mushiro,     // mushiro - rather (adverb)
    Toshite,     // toshite - as/in the role of
    Bakari,      // bakari - only/nothing but
    Suginai,     // suginai - no more than
    Ppoi,        // ppoi - ish/like
    Sudeni,      // sudeni - already (adverb)
    Oite,        // oite/ni oite - at/in
    Hodo,        // hodo - extent/degree
    Tsui,        // tsui - accidentally/unintentionally (adverb)
    Furi,        // furi - pretend
    Mama,        // mama - as is/remain
    TameNi,      // tame ni - in order to/for
    NaiUchiNi,   // nai uchi ni - before/while not
    ToShitara,   // to shitara - if we assume
    TsumorideDe, // tsumori de - with intention of
    NiKansuru,   // ni kansuru - regarding/about
    Doushitemo,  // doushitemo - no matter what (adverb)
    ToTomoni,    // to tomo ni - together with/as
    Kiri,        // kiri - only/since
    NiYotte,     // ni yotte - depending on/by
    YouNi,       // you ni - so that/in order to (without suru/naru)
    TeHajimete,  // te hajimete - for the first time after
    Gachi,       // gachi - tend to
    Zu,          // zu - without doing
    YouNiNaru,   // you ni naru - come to/become
    Seizei,      // seizei - at most/at best (adverb)
    WoHajime,    // wo hajime - beginning with
    BaHodo,      // ba hodo - the more ~ the more
    Tate,        // tate - freshly/just done
    Douyara,     // douyara - apparently/it seems (adverb)
    Kaette,      // kaette - on the contrary (adverb)
    SaeBa,       // sae ba - if only/as long as
    Koso,        // koso - emphasis particle
    Sarani,      // sarani - furthermore (adverb)
    Kawari,      // kawari - instead of/in place of
    Mai,         // mai - will not/intention not to
    Wazawaza,    // wazawaza - expressly/going out of one's way (adverb)
    Kagiru,      // kagiru - limited to/best
    OkageDe,     // okage de - thanks to
    Sae,         // sae - even
    TaMonoDa,    // ta mono da - used to
    Teki,        // teki - ish/like suffix
    Ittai,       // ittai - on earth/what (adverb)
    NiChigainai, // ni chigainai - must be/no doubt
    Nakanaka,    // nakanaka - quite/not easily (adverb)
    TaBakari,    // ta bakari - just did
}
