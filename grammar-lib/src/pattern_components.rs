// Reusable pattern components

use crate::pattern_matcher::{CustomMatcher, TokenMatcher};

// Combinator utilities

/// Concatenate multiple token sequences into one
pub fn concat(parts: Vec<Vec<TokenMatcher>>) -> Vec<TokenMatcher> {
    parts.into_iter().flatten().collect()
}

/// Wrap all tokens in a sequence as Optional (can be skipped during matching)
pub fn optional(tokens: Vec<TokenMatcher>) -> Vec<TokenMatcher> {
    tokens
        .into_iter()
        .map(|t| TokenMatcher::Optional(Box::new(t)))
        .collect()
}

/// Wrap a single matcher as Optional
#[allow(dead_code)]
pub fn optional_single(matcher: TokenMatcher) -> TokenMatcher {
    TokenMatcher::Optional(Box::new(matcher))
}

// Core verb constructions

/// Verb stem (verb in 連用形 or 連用タ接続)
pub fn verb_stem() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm)]
}

/// Te/De particle (て or で as particle)
pub fn te_particle() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Custom(CustomMatcher::TeDeForm)]
}

/// Base te-construction: Verb + て/で
pub fn te_construction() -> Vec<TokenMatcher> {
    concat(vec![verb_stem(), te_particle()])
}

/// Te-iru construction: Verb + て + いる (progressive/current state)
pub fn te_iru() -> Vec<TokenMatcher> {
    concat(vec![
        te_construction(),
        vec![TokenMatcher::specific_verb("いる")],
    ])
}

/// Te-aru construction: Verb + て + ある (resultant state)
pub fn te_aru() -> Vec<TokenMatcher> {
    concat(vec![
        te_construction(),
        vec![TokenMatcher::specific_verb("ある")],
    ])
}

/// Te-oku construction: Verb + て + おく (preparation/advance action)
pub fn te_oku() -> Vec<TokenMatcher> {
    concat(vec![
        te_construction(),
        vec![TokenMatcher::specific_verb("おく")],
    ])
}

/// Te-miru construction: Verb + て + みる (trying/testing)
pub fn te_miru() -> Vec<TokenMatcher> {
    concat(vec![
        te_construction(),
        vec![TokenMatcher::specific_verb("みる")],
    ])
}

/// Te-shimau construction: Verb + て + しまう (completion/regret)
pub fn te_shimau() -> Vec<TokenMatcher> {
    concat(vec![
        te_construction(),
        vec![TokenMatcher::specific_verb("しまう")],
    ])
}

/// Te-ageru construction: Verb + て + あげる (doing favor for someone)
pub fn te_ageru() -> Vec<TokenMatcher> {
    concat(vec![
        te_construction(),
        vec![TokenMatcher::specific_verb("あげる")],
    ])
}

/// Te-kureru construction: Verb + て + くれる (receiving favor)
pub fn te_kureru() -> Vec<TokenMatcher> {
    concat(vec![
        te_construction(),
        vec![TokenMatcher::specific_verb("くれる")],
    ])
}

/// Te-morau construction: Verb + て + もらう (asking/receiving favor)
pub fn te_morau() -> Vec<TokenMatcher> {
    concat(vec![
        te_construction(),
        vec![TokenMatcher::specific_verb("もらう")],
    ])
}

/// Te-kudasai construction: Verb + て + ください (polite request)
pub fn te_kudasai_construction() -> Vec<TokenMatcher> {
    concat(vec![
        te_construction(),
        vec![TokenMatcher::Surface("ください")],
    ])
}

// Tai-form constructions

/// Tai-form base: Verb連用形 + たい (desire)
pub fn tai_base() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("連用形"),
        TokenMatcher::Custom(CustomMatcher::TaiForm),
    ]
}

/// Tai-form full: Verb連用形 + たい
pub fn tai_form() -> Vec<TokenMatcher> {
    tai_base()
}

/// Takatta-form: Verb連用形 + たかった (past desire)
pub fn takatta_form() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![TokenMatcher::Custom(CustomMatcher::TakattaForm)],
        vec![TokenMatcher::Surface("た")],
    ])
}

/// Takunai-form: Verb連用形 + たくない (negative desire)
pub fn takunai_form() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![TokenMatcher::Custom(CustomMatcher::TakuForm)],
        vec![TokenMatcher::Surface("ない")],
    ])
}

// Request & permission constructions

/// Te-mo-ii construction: Verb + て + も + いい (permission)
pub fn te_mo_ii() -> Vec<TokenMatcher> {
    concat(vec![
        te_construction(),
        vec![TokenMatcher::Surface("も")],
        vec![TokenMatcher::Custom(CustomMatcher::IiForm)],
    ])
}

/// Te-wa-ikenai construction: Verb + て + は + いけない (prohibition)
pub fn te_wa_ikenai() -> Vec<TokenMatcher> {
    concat(vec![
        te_construction(),
        vec![TokenMatcher::Surface("は")],
        vec![TokenMatcher::Custom(CustomMatcher::IkenaiForm)],
    ])
}

/// Naide-kudasai construction: Verb未然形 + ない + で + ください (negative request)
pub fn naide_kudasai() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("未然形"),
        TokenMatcher::Surface("ない"),
        TokenMatcher::Surface("で"),
        TokenMatcher::Surface("ください"),
    ]
}

// Predicate types (for ので, から, etc.)

/// Verb predicate: Any verb form that can serve as predicate
#[allow(dead_code)]
pub fn verb_predicate() -> Vec<TokenMatcher> {
    vec![TokenMatcher::verb_with_form("基本形")]
}

/// I-adjective predicate
pub fn i_adjective_predicate() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Adjective { base_form: None }]
}

/// Na-adjective predicate: Na-adj + な (copula)
#[allow(dead_code)]
pub fn na_adjective_predicate() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Adjective { base_form: None },
        TokenMatcher::Surface("な"),
    ]
}

/// Nominal predicate: Any noun/na-adj + な (copula)
pub fn nominal_predicate() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Any, TokenMatcher::Surface("な")]
}

// Common suffixes & particles

/// ので suffix (because/so)
pub fn node_suffix() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("ので")]
}

/// から suffix (because/since)
pub fn kara_suffix() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("から")]
}

/// つもり (intention)
pub fn tsumori_suffix() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("つもり"),
        TokenMatcher::Surface("です"),
    ]
}

/// Tsumori-desu: Verb基本形 + つもり + です (intention/planning to)
pub fn tsumori_desu() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        tsumori_suffix(),
    ])
}

/// ほうがいい (should/ought to)
pub fn hou_ga_ii_suffix() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("ほう"),
        TokenMatcher::Surface("が"),
        TokenMatcher::Custom(CustomMatcher::IiForm),
    ]
}

/// Masu ending: Verb連用形 + ます
pub fn masu_ending() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("連用形"),
        TokenMatcher::Surface("ます"),
    ]
}

/// Negative: Verb未然形 + ない
pub fn negative_ending() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("未然形"),
        TokenMatcher::Surface("ない"),
    ]
}

// Node pattern variations (ので constructions)

/// Node-verb: Optional(Verb + て) + いる + ので (reason/cause with verb)
pub fn node_verb() -> Vec<TokenMatcher> {
    concat(vec![
        optional(te_construction()),
        vec![TokenMatcher::verb_with_form("基本形")],
        node_suffix(),
    ])
}

/// Node-adjective: Adjective + ので (reason/cause with i-adjective)
pub fn node_adjective() -> Vec<TokenMatcher> {
    concat(vec![i_adjective_predicate(), node_suffix()])
}

/// Node-nominal: Nominal + な + ので (reason/cause with noun/na-adjective)
pub fn node_nominal() -> Vec<TokenMatcher> {
    concat(vec![nominal_predicate(), node_suffix()])
}

// Conditional & hypothetical (N4+)

/// Ba-conditional: Verb仮定形 + ば (if/when)
pub fn ba_conditional() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("仮定形"),
        TokenMatcher::Surface("ば"),
    ]
}

/// Tara-conditional: Verb + たら (if/when - past conditional)
pub fn tara_conditional() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm)],
        vec![TokenMatcher::Custom(CustomMatcher::TaraForm)],
    ])
}

/// Nara conditional: なら (if)
pub fn nara_conditional() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("なら")]
}

// Voice & mood (N4+)

/// Potential form: Verb未然形 + れる/られる (can do)
pub fn potential_godan() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Custom(CustomMatcher::GodanMizen),
        TokenMatcher::Custom(CustomMatcher::EruForm),
    ]
}

/// Passive form (ichidan): Verb未然形 + られる (is done by)
pub fn passive_ichidan() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Custom(CustomMatcher::IchidanMizen),
        TokenMatcher::Custom(CustomMatcher::RareruForm),
    ]
}

/// Passive form (godan): Verb未然形 + れる (is done by)
pub fn passive_godan() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Custom(CustomMatcher::GodanMizen),
        TokenMatcher::Custom(CustomMatcher::ReruForm),
    ]
}

/// Causative form: Verb未然形 + せる/させる (make/let someone do)
pub fn causative() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("未然形"),
        TokenMatcher::Custom(CustomMatcher::CausativeForm),
    ]
}

/// Volitional: Verb意志形 (let's/will) - alternative to volitional_u_form
#[allow(dead_code)]
pub fn volitional() -> Vec<TokenMatcher> {
    vec![TokenMatcher::verb_with_form("意志形")]
}

/// Imperative: Verb命令形 (command/order)
pub fn imperative() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Custom(CustomMatcher::ImperativeForm)]
}

// Difficulty & ease (N4+)

/// Yasui: Verb連用形 + やすい (easy to)
pub fn yasui() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![TokenMatcher::Surface("やすい")],
    ])
}

/// Nikui: Verb連用形 + にくい (hard to)
pub fn nikui() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![TokenMatcher::Surface("にくい")],
    ])
}

// Simultaneity & progression (N4+)

/// Nagara: Verb連用形 + ながら (while doing)
pub fn nagara() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![TokenMatcher::Surface("ながら")],
    ])
}

// Additional suffixes & forms (N4+)

/// Nasai: Verb連用形 + なさい (polite command)
pub fn nasai() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![TokenMatcher::Surface("なさい")],
    ])
}

/// Tari-suru (single): Verb + たり/だり + する (single action with implied alternatives, past tense)
pub fn tari_suru_single() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm)],
        vec![TokenMatcher::Custom(CustomMatcher::TariParticle)],
        vec![TokenMatcher::specific_verb("する")],
    ])
}

/// Tari-suru (multiple): Verb + たり + ... + たり + する (listing multiple alternative actions)
pub fn tari_suru() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm)],
        vec![TokenMatcher::Custom(CustomMatcher::TariParticle)],
        vec![TokenMatcher::Wildcard {
            min: 0,
            max: 15,
            stop_conditions: vec![],
        }],
        vec![TokenMatcher::Custom(CustomMatcher::TariParticle)],
        vec![TokenMatcher::specific_verb("する")],
    ])
}

/// Potential (lexicalized godan): が particle + [optional modifiers] + ichidan verb
/// Examples: 水が飲める (can drink water), 空が見えない (can't see the sky)
pub fn potential_ga_verb() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("が"),
        TokenMatcher::Wildcard {
            min: 0,
            max: 2,
            stop_conditions: vec![
                TokenMatcher::Verb {
                    conjugation_form: None,
                    base_form: None,
                },
                TokenMatcher::Custom(CustomMatcher::Particle),
            ],
        },
        TokenMatcher::Custom(CustomMatcher::GaPotentialVerb),
    ]
}

/// Potential (ichidan with が particle): が + [optional modifiers] + Verb未然形 + られる
/// Examples: 魚が食べられる (can eat fish), 空が見える (can see sky)
/// More specific than bare potential_ichidan - requires が as evidence
pub fn potential_ga_ichidan() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("が"),
        TokenMatcher::Wildcard {
            min: 0,
            max: 2,
            stop_conditions: vec![
                TokenMatcher::Verb {
                    conjugation_form: None,
                    base_form: None,
                },
                TokenMatcher::Custom(CustomMatcher::Particle),
            ],
        },
        TokenMatcher::Custom(CustomMatcher::IchidanMizen),
        TokenMatcher::Custom(CustomMatcher::RareruForm),
    ]
}

/// Volitional alternative: Verb未然ウ接続 + う (let's/I will)
pub fn volitional_u_form() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("未然ウ接続"),
        TokenMatcher::Surface("う"),
    ]
}

/// Past negative: Verb未然形 + なかったForm + た (was not)
pub fn past_negative() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("未然形")],
        vec![TokenMatcher::Custom(CustomMatcher::NakattaForm)],
        vec![TokenMatcher::Surface("た")],
    ])
}

/// Te + mo: Verb + て + も (even if)
pub fn te_mo() -> Vec<TokenMatcher> {
    concat(vec![te_construction(), vec![TokenMatcher::Surface("も")]])
}

/// Naide: Verb未然形 + ない + で (don't/without doing)
/// Uses NonPotentialMizen to exclude potential forms like 帰れないで
pub fn naide() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Custom(CustomMatcher::NonPotentialMizen),
        TokenMatcher::Surface("ない"),
        TokenMatcher::Surface("で"),
    ]
}

/// Te-sumimasen: Verb + て + すみません (apology for inconvenience)
pub fn te_sumimasen() -> Vec<TokenMatcher> {
    concat(vec![
        te_construction(),
        vec![TokenMatcher::Surface("すみません")],
    ])
}

/// Te-kurete-arigatou: Verb + て + くれる + て + ありがとう (thanks for doing)
pub fn te_kurete_arigatou() -> Vec<TokenMatcher> {
    concat(vec![
        te_construction(),
        vec![TokenMatcher::specific_verb("くれる")],
        vec![TokenMatcher::Custom(CustomMatcher::TeDeForm)],
        vec![TokenMatcher::Surface("ありがとう")],
    ])
}

/// Te-yokatta: Verb + て + よかった (glad that)
pub fn te_yokatta() -> Vec<TokenMatcher> {
    concat(vec![
        te_construction(),
        vec![TokenMatcher::Custom(CustomMatcher::YokattaForm)],
    ])
}

/// Nakute-mo-ii: Verb未然形 + なく + て + も + いい (it's okay not to)
pub fn nakute_mo_ii() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("未然形")],
        vec![TokenMatcher::Custom(CustomMatcher::NakuForm)],
        vec![TokenMatcher::Surface("て")],
        vec![TokenMatcher::Surface("も")],
        vec![TokenMatcher::Custom(CustomMatcher::IiForm)],
    ])
}

/// Ba-yokatta: Verb仮定形 + ば + よかった + た (wish/regret)
pub fn ba_yokatta() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("仮定形")],
        vec![TokenMatcher::Surface("ば")],
        vec![TokenMatcher::Custom(CustomMatcher::YokattaForm)],
        vec![TokenMatcher::Surface("た")],
    ])
}

/// Hazu-desu: Verb基本形 + はず + です (should be/supposed to)
pub fn hazu_desu() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("はず")],
        vec![TokenMatcher::Surface("です")],
    ])
}

/// Koto-ni-suru: Verb基本形 + こと + に + する (decide to do)
pub fn koto_ni_suru() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("こと")],
        vec![TokenMatcher::Surface("に")],
        vec![TokenMatcher::specific_verb("する")],
    ])
}

/// Koto-ni-naru: Verb基本形 + こと + に + なる (come to do/happen to)
pub fn koto_ni_naru() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("こと")],
        vec![TokenMatcher::Surface("に")],
        vec![TokenMatcher::specific_verb("なる")],
    ])
}

/// Noni: Verb基本形 + のに (despite/in order to)
pub fn noni() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("のに")],
    ])
}

/// Mitai: Verb基本形 + みたい (looks like/seems like)
pub fn mitai() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("みたい")],
    ])
}

/// Kamo-shirenai: Verb基本形 + かも + しれる + ない (might not)
pub fn kamo_shirenai() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("かも")],
        vec![TokenMatcher::specific_verb("しれる")],
        vec![TokenMatcher::Surface("ない")],
    ])
}

/// Kamo-shiremasen: Verb基本形 + かも + しれる + ませ + ん (might not - polite)
pub fn kamo_shiremasen() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("かも")],
        vec![TokenMatcher::specific_verb("しれる")],
        vec![TokenMatcher::Surface("ませ")],
        vec![TokenMatcher::Surface("ん")],
    ])
}

/// Te-itadakemasen-ka: Verb + て + いただく (request to do - polite negative question)
pub fn te_itadakemasen_ka() -> Vec<TokenMatcher> {
    concat(vec![
        te_construction(),
        vec![TokenMatcher::specific_verb("いただく")],
    ])
}

/// Ga-hoshii: が + ほしい (want something)
pub fn ga_hoshii() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("が"), TokenMatcher::Surface("ほしい")]
}

/// Shika-nai: しか + Verb未然形 + ない (only/nothing but)
pub fn shika_nai() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Surface("しか"),
        TokenMatcher::Custom(CustomMatcher::NonNaruMizen),
        TokenMatcher::Surface("ない"),
    ]
}

/// To-iu: という (called/named/in other words)
pub fn to_iu() -> Vec<TokenMatcher> {
    vec![TokenMatcher::Surface("という")]
}

/// You-ni-suru: Verb基本形 + よう + に + する (try to/make an effort to)
pub fn you_ni_suru() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("よう")],
        vec![TokenMatcher::Surface("に")],
        vec![TokenMatcher::specific_verb("する")],
    ])
}

/// You-ni-naru: Verb基本形 + よう + に + なる (come to/gradually)
pub fn you_ni_naru() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("よう")],
        vec![TokenMatcher::Surface("に")],
        vec![TokenMatcher::specific_verb("なる")],
    ])
}

/// Tame-ni: Verb基本形 + ため + に (for the purpose of/in order to)
pub fn tame_ni() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("ため")],
        vec![TokenMatcher::Surface("に")],
    ])
}

/// Zu: Verb未然形 + ず (without doing/don't)
pub fn zu() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::verb_with_form("未然形"),
        TokenMatcher::Surface("ず"),
    ]
}

/// Ta-form: Verb + た/だ (past tense - for use in other patterns)
pub fn ta_form() -> Vec<TokenMatcher> {
    vec![
        TokenMatcher::Custom(CustomMatcher::FlexibleVerbForm),
        TokenMatcher::Custom(CustomMatcher::PastAuxiliary),
    ]
}

/// Ta-bakari: Verb + た + ばかり (just did/had just)
pub fn ta_bakari() -> Vec<TokenMatcher> {
    concat(vec![ta_form(), vec![TokenMatcher::Surface("ばかり")]])
}

/// Ta-mono-da: Verb + た + もの + だ (used to/nostalgic)
pub fn ta_mono_da() -> Vec<TokenMatcher> {
    concat(vec![
        ta_form(),
        vec![TokenMatcher::Surface("もの")],
        vec![TokenMatcher::Surface("だ")],
    ])
}

/// Ta-mono-desu: Verb + た + もの + です (polite: used to)
pub fn ta_mono_desu() -> Vec<TokenMatcher> {
    concat(vec![
        ta_form(),
        vec![TokenMatcher::Surface("もの")],
        vec![TokenMatcher::Surface("です")],
    ])
}

/// Ta-koto-ga-aru: Verb + た + こと + が + ある (have done before)
pub fn ta_koto_ga_aru() -> Vec<TokenMatcher> {
    concat(vec![
        ta_form(),
        vec![TokenMatcher::Surface("こと")],
        vec![TokenMatcher::Surface("が")],
        vec![TokenMatcher::specific_verb("ある")],
    ])
}

/// Ta-ue-de: Verb + た + 上 + で (after doing/upon doing)
pub fn ta_ue_de() -> Vec<TokenMatcher> {
    concat(vec![
        ta_form(),
        vec![TokenMatcher::Surface("上")],
        vec![TokenMatcher::Surface("で")],
    ])
}

/// Te-request: Verb + て + くださる連用形 (polite request)
pub fn te_request() -> Vec<TokenMatcher> {
    concat(vec![
        te_construction(),
        vec![TokenMatcher::specific_verb_with_form("くださる", "連用形")],
    ])
}

/// Dictionary-to: Verb基本形 + と (quotative/conditional particle)
pub fn dictionary_to() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("と")],
    ])
}

/// To-ii: Verb基本形 + と + いい (hope/wish)
pub fn to_ii() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("と")],
        vec![TokenMatcher::Custom(CustomMatcher::ToIiForm)],
    ])
}

/// Ka-dou-ka: Verb基本形 + か + どう + か (whether or not)
pub fn ka_dou_ka() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("か")],
        vec![TokenMatcher::Surface("どう")],
        vec![TokenMatcher::Surface("か")],
    ])
}

/// Hajimeru: Verb連用形 + 始める (start doing/begin)
pub fn hajimeru() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![TokenMatcher::specific_verb("始める")],
    ])
}

/// Kaneru: Verb連用形 + かねる (cannot/have trouble doing)
pub fn kaneru() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![TokenMatcher::specific_verb("かねる")],
    ])
}

/// Nai-uchi-ni: Verb未然形 + ない + うち + に (while not yet)
pub fn nai_uchi_ni() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("未然形")],
        vec![TokenMatcher::Surface("ない")],
        vec![TokenMatcher::Surface("うち")],
        vec![TokenMatcher::Surface("に")],
    ])
}

/// Yamuoezu-verb: やむをえる + ず (unavoidably/whether one likes it or not)
pub fn yamuoezu_verb() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::specific_verb("やむをえる")],
        vec![TokenMatcher::Surface("ず")],
    ])
}

/// Ni-iku: Verb連用形 + に + 行く (go to do something)
pub fn ni_iku() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("連用形")],
        vec![TokenMatcher::Surface("に")],
        vec![TokenMatcher::specific_verb("行く")],
    ])
}

/// Mae-ni: Verb基本形 + 前 + に (before doing)
pub fn mae_ni() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("基本形")],
        vec![TokenMatcher::Surface("前")],
        vec![TokenMatcher::Surface("に")],
    ])
}

/// Sugiru: SugiruStem + すぎる (too much/excessively)
pub fn sugiru() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::Custom(CustomMatcher::SugiruStem)],
        vec![TokenMatcher::specific_verb("すぎる")],
    ])
}

/// N-desu: NDesuForm + です (explanatory んです)
pub fn n_desu() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::Custom(CustomMatcher::NDesuForm)],
        vec![TokenMatcher::Surface("です")],
    ])
}

/// Causative-passive: Verb未然形 + SaseForm + RareruForm (made to do passive)
pub fn causative_passive() -> Vec<TokenMatcher> {
    concat(vec![
        vec![TokenMatcher::verb_with_form("未然形")],
        vec![TokenMatcher::Custom(CustomMatcher::SaseForm)],
        vec![TokenMatcher::Custom(CustomMatcher::RareruForm)],
    ])
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat_combines_sequences() {
        let part1 = vec![TokenMatcher::Surface("a")];
        let part2 = vec![TokenMatcher::Surface("b")];
        let result = concat(vec![part1, part2]);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_optional_wraps_tokens() {
        let tokens = vec![TokenMatcher::Surface("test")];
        let result = optional(tokens);
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], TokenMatcher::Optional(_)));
    }

    #[test]
    fn test_te_construction_has_two_parts() {
        let tokens = te_construction();
        assert_eq!(tokens.len(), 2);
    }

    #[test]
    fn test_te_iru_has_three_parts() {
        let tokens = te_iru();
        assert_eq!(tokens.len(), 3);
    }

    #[test]
    fn test_tai_form_has_two_parts() {
        let tokens = tai_form();
        assert_eq!(tokens.len(), 2);
    }

    #[test]
    fn test_takatta_form_has_three_parts() {
        let tokens = takatta_form();
        assert_eq!(tokens.len(), 3);
    }

    #[test]
    fn test_composed_pattern() {
        // Example: optional(te_iru()) + ので
        let tokens = concat(vec![optional(te_iru()), node_suffix()]);
        // Should have: Optional(FlexibleVerbForm), Optional(TeDeForm), Optional(いる), ので
        assert_eq!(tokens.len(), 4);
    }

    #[test]
    fn test_optional_single_matcher() {
        let matcher = optional_single(TokenMatcher::Surface("test"));
        assert!(matches!(matcher, TokenMatcher::Optional(_)));
    }

    #[test]
    fn test_naide_kudasai_has_four_parts() {
        let tokens = naide_kudasai();
        assert_eq!(tokens.len(), 4);
    }

    #[test]
    fn test_te_mo_ii_has_four_parts() {
        let tokens = te_mo_ii();
        assert_eq!(tokens.len(), 4);
    }
}
