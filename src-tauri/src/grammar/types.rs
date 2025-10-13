use serde::{Deserialize, Serialize};

/// Kagome token structure from morphological analysis
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KagomeToken {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub start: u32,
    #[serde(default)]
    pub end: u32,
    #[serde(default)]
    pub surface: String,
    #[serde(default)]
    pub class: String,
    pub pos: Vec<String>,
    pub base_form: String,
    pub reading: String,
    #[serde(default)]
    pub pronunciation: String,
    #[serde(default)]
    pub features: Vec<String>,
}

/// Grammar conjugation patterns recognized by the pattern matcher
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
    // N2 patterns
    Toutei,             // toutei - cannot possibly (adverb)
    Yoppodo,            // yoppodo - very/considerably (adverb)
    Iyoiyo,             // iyoiyo - finally/at last (adverb)
    Sekkaku,            // sekkaku - with trouble/specially (adverb)
    Yamuoezu,           // yamuoezu - unavoidably (adverb)
    Yappari,            // yappari - as expected/after all (adverb)
    Narubeku,           // narubeku - as much as possible (adverb)
    Tashika,            // tashika - if I remember correctly (adverb)
    ManIchi,            // man'ichi - by any chance (adverb)
    Nanishiro,          // nanishiro - at any rate/after all (adverb)
    SoreniShitemo,      // sore ni shitemo - nevertheless/even so
    Tachimachi,         // tachimachi - at once/in no time (adverb)
    Sasugani,           // sasugani - as expected/even (adverb)
    Itsunomanika,       // itsu no ma ni ka - before one knows
    Aete,               // aete - dare to/purposely (adverb)
    Semete,             // semete - at least (adverb)
    Nantoittemo,        // nanto ittemo - after all/no matter what
    RouNi,              // roku ni - properly/well (adverb)
    Kaneru,             // kaneru - cannot/unable to
    Kanenai,            // kanenai - might/could possibly
    Tamaranai,          // tamaranai - unbearable/can't help
    NaideSumu,          // naide sumu - can get by without
    KaraNaru,           // kara naru - consist of
    YoriShikataGanai,   // yori shikata ga nai - cannot help but
    TaUeDe,             // ta ue de - after doing
    NiAtaru,            // ni ataru - correspond to/be equivalent
    Gotoshi,            // gotoshi - like/as if
    Tsuujite,           // tsuujite - through/throughout
    Noboru,             // noboru - amount to/reach
    Gatera,             // gatera - while/on the occasion of
    Oyobi,              // oyobi - and/as well as
    SeiKa,              // sei ka - perhaps because
    Yueni,              // yueni - therefore/because
    IppouDewa,          // ippou dewa - on the other hand
    MonoNo,             // mono no - although/though
    KuseNi,             // kuse ni - and yet/even though
    KaketeHa,           // kakete ha - when it comes to
    ItaruMade,          // itaru made - up to/as far as
    IgaiNo,             // igai no - other than/except
    BaIiNoni,           // ba ii noni - if only/I wish
    WakeDesu,           // wake desu - it is that/the fact is
    YouNaKiGaSuru,      // you na ki ga suru - feel like/have a feeling
    NiKiWoTsukeru,      // ni ki wo tsukeru - be careful of
    BetsuniNai,         // betsuni~nai - not particularly
    WakeNiwaIkanai,     // wake niwa ikanai - cannot afford to
    DewaNaiDarouKa,     // dewa nai darou ka - isn't it that
    ToIuWakeDewaNai,    // to iu wake dewa nai - it doesn't mean that
    NiKoshitaKotoWaNai, // ni koshita koto wa nai - nothing is better than
    Sashitsukaenai,     // sashitsukaenai - no problem/no objection
    NaiWakeNiwaIkanai,  // nai wake niwa ikanai - cannot not do/must
    ToIttemo,           // to ittemo - even though I say/although
    GaKiNiNaru,         // ga ki ni naru - worry about/be concerned
    OmouYouni,          // omou you ni - as one wishes
    MonoDesukara,       // mono desukara - because/since
    // N1 patterns
    Meku,               // meku - show signs of/like
    Mamire,             // mamire - covered with/smeared with
    Zukume,             // zukume - entirely/nothing but
    Ppanashi,           // ppanashi - leaving as is
    Kiwamaru,           // kiwamaru - extremely
    Beku,               // beku - in order to/for
    Bekarazu,           // bekarazu - must not/should not
    Majiki,             // majiki - should not/unworthy
    Nari,               // nari - as soon as
    YaInaya,            // ya inaya - as soon as
    GaHayaiKa,          // ga hayai ka - as soon as
    GaSaigo,            // ga saigo - once ~ forever
    Gotoki,             // gotoki - like/such as
    WoKawakiriNi,       // wo kawakiri ni - starting with
    WoMotte,            // wo motte - with/by means of
    Nakushiteha,        // nakushiteha - without
    Nashini,            // nashini - without
    Naradewa,           // naradewa - unique to
    NiTaru,             // ni taru - worth/deserve
    Toatte,             // toatte - because/being
    Katagata,           // katagata - while/also to
    WoKagiriNi,         // wo kagiri ni - as the last time
    WoHete,             // wo hete - through/via
    WoOshite,           // wo oshite - in spite of
    WoFumaete,          // wo fumaete - based on
    TeYamanai,          // te yamanai - never cease
    ToOmoikiya,         // to omoikiya - contrary to expectations
    ToAreba,            // to areba - if it's the case
    TaTokoroDe,         // ta tokoro de - even if
    DeAre,              // de are - whether/even if
    ToWaIe,             // to wa ie - although/even though
    YouGa,              // you ga - no matter/even if
    NaiMadeMo,          // nai made mo - even if not
    NagaraMo,           // nagara mo - while/though
    DewaArumaishi,      // dewa arumaishi - it's not like
    ToShitaTokoroDe,    // to shita tokoro de - even if
    ToIedomo,           // to iedomo - even though/although
    TomoNaruto,         // tomo naruto - when it comes to
    NiTaenai,           // ni taenai - cannot bear/unbearable
    NiSokushite,        // ni sokushite - in accordance with
    ToAimatte,          // to aimatte - combined with
    WoYosoni,           // wo yosoni - in spite of/ignoring
    TemoSashitsukaenai, // temo sashitsukaenai - it's okay to
    WoKinjienai,        // wo kinjienai - cannot help but
    WoYoginakusareru,   // wo yoginakusareru - be forced to
    TeKaratoiumono,     // te karatoiumono - since/ever since
    NimoMashite,        // nimo mashite - more than/even more
    NiHikikae,          // ni hikikae - in contrast to
    Ikan,               // ikan - depending on
    Taritomo,           // taritomo - even/not even
    KiraiGaAru,         // kirai ga aru - tend to/have tendency
    ShimatsuDa,         // shimatsu da - end up/come to
    Warini,             // warini - considering/for
    KaiMoNaku,          // kai mo naku - despite efforts/in vain
    DakeMashi,          // dake mashi - at least/better than
    NaideWaSumanai,     // naide wa sumanai - cannot get away with
    KotoNashini,        // koto nashini - without
    DeSura,             // de sura - even
    Nagarani,           // nagarani - while remaining/as
    HaOroka,            // ha oroka - let alone/not to mention
}
