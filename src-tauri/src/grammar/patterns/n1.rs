use crate::grammar::pattern_matcher::{GrammarPattern, TokenMatcher};
use crate::grammar::types::ConjugationPattern;

/// JLPT N1 level grammar patterns (advanced forms)
pub fn get_patterns() -> Vec<(GrammarPattern, ConjugationPattern, &'static str)> {
    vec![
        // Phase 1: Suffix Patterns (9 patterns)
        // めく: Shows signs of, appears to be (suffix to noun/verb stem)
        // Examples: 謎めく (mysterious), 春めく (spring-like), 皮肉めく (ironic)
        // Note: Kagome may tokenize as single verb (春めく) or split (謎 + めく)
        // We match the split version; compound versions recognized as single verbs won't match
        (
            GrammarPattern {
                name: "meku",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("めく")],
                priority: 6,
            },
            ConjugationPattern::Meku,
            "n1",
        ),
        // まみれ: Covered with, smeared with (suffix to noun)
        // Examples: 血まみれ (covered in blood), 泥まみれ (covered in mud)
        (
            GrammarPattern {
                name: "mamire",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("まみれ")],
                priority: 6,
            },
            ConjugationPattern::Mamire,
            "n1",
        ),
        // ずくめ: Entirely, nothing but (suffix to noun)
        // Examples: 黒ずくめ (all in black), いいことずくめ (nothing but good things)
        (
            GrammarPattern {
                name: "zukume",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("ずくめ")],
                priority: 6,
            },
            ConjugationPattern::Zukume,
            "n1",
        ),
        // っぱなし: Leaving as is, leaving undone (suffix to verb stem)
        // Examples: やりっぱなし (leaving things undone), 付けっぱなし (leaving on)
        (
            GrammarPattern {
                name: "ppanashi",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("っぱなし")],
                priority: 6,
            },
            ConjugationPattern::Ppanashi,
            "n1",
        ),
        // 極まる/極まりない: Extremely, to the utmost (suffix to na-adjective stem)
        // Examples: 失礼極まる (extremely rude), 危険極まりない (extremely dangerous)
        (
            GrammarPattern {
                name: "kiwamaru",
                tokens: vec![TokenMatcher::Any, TokenMatcher::specific_verb("極まる")],
                priority: 7,
            },
            ConjugationPattern::Kiwamaru,
            "n1",
        ),
        // 極まりない: Kagome lexicalizes this as single adjective, not 極まる+ない
        (
            GrammarPattern {
                name: "kiwamarinai",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("極まりない")],
                priority: 7,
            },
            ConjugationPattern::Kiwamaru,
            "n1",
        ),
        // べく: In order to, for the purpose of (suffix to verb stem)
        // Examples: 見るべく (in order to see), 知るべく (in order to know)
        (
            GrammarPattern {
                name: "beku",
                tokens: vec![TokenMatcher::verb_with_form("基本形"), TokenMatcher::Surface("べく")],
                priority: 7,
            },
            ConjugationPattern::Beku,
            "n1",
        ),
        // べからず: Must not, should not (classical prohibition, suffix to verb stem)
        // Examples: 入るべからず (must not enter), 触るべからず (must not touch)
        // Kagome tokenizes as べから + ず
        (
            GrammarPattern {
                name: "bekarazu",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("べから"),
                    TokenMatcher::Surface("ず"),
                ],
                priority: 8,
            },
            ConjugationPattern::Bekarazu,
            "n1",
        ),
        // まじき: Should not, unworthy of (classical negative, suffix to verb stem)
        // Examples: あるまじき (should not be), 許すまじき (unforgivable)
        (
            GrammarPattern {
                name: "majiki",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("まじき")],
                priority: 6,
            },
            ConjugationPattern::Majiki,
            "n1",
        ),
        // Phase 2: Simple Fixed Expressions (20 patterns)
        // なり: As soon as (verb + なり)
        (
            GrammarPattern {
                name: "nari",
                tokens: vec![TokenMatcher::verb_with_form("基本形"), TokenMatcher::Surface("なり")],
                priority: 7,
            },
            ConjugationPattern::Nari,
            "n1",
        ),
        // や否や: As soon as (verb + や + 否や)
        // Kagome splits kanji form as: や + 否や
        (
            GrammarPattern {
                name: "ya_inaya",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("や"),
                    TokenMatcher::Surface("否や"),
                ],
                priority: 8,
            },
            ConjugationPattern::YaInaya,
            "n1",
        ),
        // Kagome lexicalizes やいなや (kana form) as single particle
        (
            GrammarPattern {
                name: "ya_inaya_single",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("やいなや"),
                ],
                priority: 7,
            },
            ConjugationPattern::YaInaya,
            "n1",
        ),
        // が早いか: As soon as (verb + が + 早い + か)
        // Kagome splits as: verb + が + 早い (adjective) + か (particle)
        (
            GrammarPattern {
                name: "ga_hayai_ka",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("が"),
                    TokenMatcher::Surface("早い"),
                    TokenMatcher::Surface("か"),
                ],
                priority: 9,
            },
            ConjugationPattern::GaHayaiKa,
            "n1",
        ),
        // が最後: Once ~ then forever (verb + が + 最後)
        (
            GrammarPattern {
                name: "ga_saigo",
                tokens: vec![
                    TokenMatcher::verb_with_form("基本形"),
                    TokenMatcher::Surface("が"),
                    TokenMatcher::Surface("最後"),
                ],
                priority: 8,
            },
            ConjugationPattern::GaSaigo,
            "n1",
        ),
        // ごとき: Like/such as (noun + ごとき)
        (
            GrammarPattern {
                name: "gotoki",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("ごとき")],
                priority: 6,
            },
            ConjugationPattern::Gotoki,
            "n1",
        ),
        // を皮切りに: Starting with (noun + を + 皮切りに)
        (
            GrammarPattern {
                name: "wo_kawakiri_ni",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("を"),
                    TokenMatcher::Surface("皮切り"),
                    TokenMatcher::Surface("に"),
                ],
                priority: 9,
            },
            ConjugationPattern::WoKawakiriNi,
            "n1",
        ),
        // をもって: With/by means of (noun + をもって)
        // Kagome returns をもって as single token
        (
            GrammarPattern {
                name: "wo_motte",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("をもって")],
                priority: 7,
            },
            ConjugationPattern::WoMotte,
            "n1",
        ),
        // なくしては: Without (noun + なく + し + て + は)
        // Kagome splits as: noun + なく (from ない) + し (from する) + て + は
        (
            GrammarPattern {
                name: "nakushiteha",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("なく"),
                    TokenMatcher::Surface("し"),
                    TokenMatcher::Surface("て"),
                    TokenMatcher::Surface("は"),
                ],
                priority: 10,
            },
            ConjugationPattern::Nakushiteha,
            "n1",
        ),
        // なしに: Without (noun + なしに)
        (
            GrammarPattern {
                name: "nashini",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("なし"),
                    TokenMatcher::Surface("に"),
                ],
                priority: 7,
            },
            ConjugationPattern::Nashini,
            "n1",
        ),
        // ならでは: Unique to (noun + ならでは)
        (
            GrammarPattern {
                name: "naradewa",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("ならでは"),
                ],
                priority: 7,
            },
            ConjugationPattern::Naradewa,
            "n1",
        ),
        // に足る: Worth/deserve (noun + に + 足る)
        (
            GrammarPattern {
                name: "ni_taru",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("に"),
                    TokenMatcher::specific_verb("足る"),
                ],
                priority: 8,
            },
            ConjugationPattern::NiTaru,
            "n1",
        ),
        // とあって: Because/being (noun + と + あっ + て)
        // Kagome splits as: noun + と + あっ (verb ある) + て
        (
            GrammarPattern {
                name: "toatte",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("と"),
                    TokenMatcher::Surface("あっ"),
                    TokenMatcher::Surface("て"),
                ],
                priority: 9,
            },
            ConjugationPattern::Toatte,
            "n1",
        ),
        // かたがた: While/also to (verb stem + かたがた)
        (
            GrammarPattern {
                name: "katagata",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("かたがた")],
                priority: 6,
            },
            ConjugationPattern::Katagata,
            "n1",
        ),
        // を限りに: As the last time (を + 限りに)
        (
            GrammarPattern {
                name: "wo_kagiri_ni",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("を"),
                    TokenMatcher::Surface("限り"),
                    TokenMatcher::Surface("に"),
                ],
                priority: 8,
            },
            ConjugationPattern::WoKagiriNi,
            "n1",
        ),
        // を経て: Through/via (を + 経て)
        (
            GrammarPattern {
                name: "wo_hete",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("を"),
                    TokenMatcher::Surface("経"),
                    TokenMatcher::Surface("て"),
                ],
                priority: 8,
            },
            ConjugationPattern::WoHete,
            "n1",
        ),
        // をおして: In spite of (を + おして)
        // Kagome lexicalizes おして as single adverb
        (
            GrammarPattern {
                name: "wo_oshite",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("を"),
                    TokenMatcher::Surface("おして"),
                ],
                priority: 8,
            },
            ConjugationPattern::WoOshite,
            "n1",
        ),
        // をふまえて: Based on (を + 踏まえて)
        (
            GrammarPattern {
                name: "wo_fumaete",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("を"),
                    TokenMatcher::Surface("踏まえ"),
                    TokenMatcher::Surface("て"),
                ],
                priority: 8,
            },
            ConjugationPattern::WoFumaete,
            "n1",
        ),
        // てやまない: Never cease (te-form + やまない)
        (
            GrammarPattern {
                name: "te_yamanai",
                tokens: vec![
                    TokenMatcher::Surface("て"),
                    TokenMatcher::Surface("やま"),
                    TokenMatcher::Surface("ない"),
                ],
                priority: 8,
            },
            ConjugationPattern::TeYamanai,
            "n1",
        ),
        // Phase 3: Conditional/Concessive Patterns (12 patterns)
        // と思いきや: Contrary to expectations (verb/adjective + と + 思い + き + や)
        // Examples: 勝つと思いきや負けた, 簡単だと思いきや難しい
        // Kagome splits as: と + 思い (verb) + き (auxiliary) + や (particle)
        (
            GrammarPattern {
                name: "to_omoikiya",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("と"),
                    TokenMatcher::Surface("思い"),
                    TokenMatcher::Surface("き"),
                    TokenMatcher::Surface("や"),
                ],
                priority: 10,
            },
            ConjugationPattern::ToOmoikiya,
            "n1",
        ),
        // とあれば: If it's the case (noun + と + あれば)
        // Examples: 君のためとあれば何でもする
        (
            GrammarPattern {
                name: "to_areba",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("と"),
                    TokenMatcher::Surface("あれ"),
                    TokenMatcher::Surface("ば"),
                ],
                priority: 9,
            },
            ConjugationPattern::ToAreba,
            "n1",
        ),
        // たところで: Even if (past tense + ところ + で)
        // Examples: 謝ったところで許されない
        // Kagome splits ところで as: ところ (noun) + で (particle)
        (
            GrammarPattern {
                name: "ta_tokoro_de",
                tokens: vec![
                    TokenMatcher::verb_with_form("連用タ接続"),
                    TokenMatcher::Surface("た"),
                    TokenMatcher::Surface("ところ"),
                    TokenMatcher::Surface("で"),
                ],
                priority: 10,
            },
            ConjugationPattern::TaTokoroDe,
            "n1",
        ),
        // であれ: Whether/even if (noun + で + あれ)
        // Examples: 誰であれ平等に扱う
        (
            GrammarPattern {
                name: "de_are",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("で"),
                    TokenMatcher::Surface("あれ"),
                ],
                priority: 7,
            },
            ConjugationPattern::DeAre,
            "n1",
        ),
        // とはいえ: Although/even though (noun/verb + と + は + いえ)
        // Examples: 子供とはいえ分かる, 忙しいとはいえ休むべき
        (
            GrammarPattern {
                name: "to_wa_ie",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("と"),
                    TokenMatcher::Surface("は"),
                    TokenMatcher::Surface("いえ"),
                ],
                priority: 9,
            },
            ConjugationPattern::ToWaIe,
            "n1",
        ),
        // ものを: If only/I wish (verb/adjective + ものを)
        // Examples: 聞けばよかったものを
        (
            GrammarPattern {
                name: "mono_wo",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("もの"),
                    TokenMatcher::Surface("を"),
                ],
                priority: 7,
            },
            ConjugationPattern::MonoWo,
            "n1",
        ),
        // ようが: No matter/even if (verb volitional + が)
        // Examples: 雨が降ろうが行く
        (
            GrammarPattern {
                name: "you_ga",
                tokens: vec![
                    TokenMatcher::verb_with_form("未然ウ接続"),
                    TokenMatcher::Surface("う"),
                    TokenMatcher::Surface("が"),
                ],
                priority: 8,
            },
            ConjugationPattern::YouGa,
            "n1",
        ),
        // ないまでも: Even if not (negative + まで + も)
        // Examples: 完璧でないまでも十分だ
        (
            GrammarPattern {
                name: "nai_made_mo",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("ない"),
                    TokenMatcher::Surface("まで"),
                    TokenMatcher::Surface("も"),
                ],
                priority: 9,
            },
            ConjugationPattern::NaiMadeMo,
            "n1",
        ),
        // ながらも: While/though (verb stem + ながら + も)
        // Examples: 知りながらも黙っていた
        (
            GrammarPattern {
                name: "nagara_mo",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("ながら"),
                    TokenMatcher::Surface("も"),
                ],
                priority: 7,
            },
            ConjugationPattern::NagaraMo,
            "n1",
        ),
        // ではあるまいし: It's not like (noun/na-adj + で + は + ある + まい + し)
        // Examples: 子供ではあるまいし分かるでしょ
        // Kagome splits as: で + は + ある (auxiliary) + まい (auxiliary) + し (particle)
        (
            GrammarPattern {
                name: "dewa_arumaishi",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("で"),
                    TokenMatcher::Surface("は"),
                    TokenMatcher::Surface("ある"),
                    TokenMatcher::Surface("まい"),
                    TokenMatcher::Surface("し"),
                ],
                priority: 11,
            },
            ConjugationPattern::DewaArumaishi,
            "n1",
        ),
        // としたところで: Even if (noun/verb + と + し + た + ところ + で)
        // Examples: 今更謝るとしたところで遅い
        // Kagome splits ところで as: ところ (noun) + で (particle)
        (
            GrammarPattern {
                name: "to_shita_tokoro_de",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("と"),
                    TokenMatcher::Surface("し"),
                    TokenMatcher::Surface("た"),
                    TokenMatcher::Surface("ところ"),
                    TokenMatcher::Surface("で"),
                ],
                priority: 11,
            },
            ConjugationPattern::ToShitaTokoroDe,
            "n1",
        ),
        // といえども: Even though/although (noun + と + いえ + ども)
        // Examples: 天才といえども努力が必要だ
        // Kagome splits as: と + いえ (verb いう conditional) + ども (particle)
        (
            GrammarPattern {
                name: "to_iedomo",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("と"),
                    TokenMatcher::Surface("いえ"),
                    TokenMatcher::Surface("ども"),
                ],
                priority: 9,
            },
            ConjugationPattern::ToIedomo,
            "n1",
        ),
        // Phase 4: Complex Multi-Word Expressions (12 patterns)
        // ともなると: When it comes to (noun + と + も + なる + と)
        // Examples: プロともなると違う
        (
            GrammarPattern {
                name: "tomo_naruto",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("と"),
                    TokenMatcher::Surface("も"),
                    TokenMatcher::Surface("なる"),
                    TokenMatcher::Surface("と"),
                ],
                priority: 10,
            },
            ConjugationPattern::TomoNaruto,
            "n1",
        ),
        // にたえない: Cannot bear/unbearable (noun + に + 堪え + ない)
        // Examples: 見るに堪えない
        (
            GrammarPattern {
                name: "ni_taenai",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("に"),
                    TokenMatcher::Surface("堪え"),
                    TokenMatcher::Surface("ない"),
                ],
                priority: 9,
            },
            ConjugationPattern::NiTaenai,
            "n1",
        ),
        // ところを: Although/in spite of (noun/verb + ところ + を)
        // Examples: 忙しいところを来てくれた
        (
            GrammarPattern {
                name: "tokoro_wo",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("ところ"),
                    TokenMatcher::Surface("を"),
                ],
                priority: 7,
            },
            ConjugationPattern::TokoroWo,
            "n1",
        ),
        // にそくして: In accordance with (noun + に + 即し + て)
        // Examples: 規則に即して行動する
        (
            GrammarPattern {
                name: "ni_sokushite",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("に"),
                    TokenMatcher::Surface("即し"),
                    TokenMatcher::Surface("て"),
                ],
                priority: 8,
            },
            ConjugationPattern::NiSokushite,
            "n1",
        ),
        // と相まって: Combined with (noun + と + 相まって)
        // Examples: 努力と相まって成功した
        // Kagome lexicalizes 相まって as single adverb token
        (
            GrammarPattern {
                name: "to_aimatte",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("と"),
                    TokenMatcher::Surface("相まって"),
                ],
                priority: 8,
            },
            ConjugationPattern::ToAimatte,
            "n1",
        ),
        // をよそに: In spite of/ignoring (noun + を + よそ + に)
        // Examples: 反対をよそに実行した
        // Kagome splits as: を + よそ (noun) + に
        (
            GrammarPattern {
                name: "wo_yosoni",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("を"),
                    TokenMatcher::Surface("よそ"),
                    TokenMatcher::Surface("に"),
                ],
                priority: 9,
            },
            ConjugationPattern::WoYosoni,
            "n1",
        ),
        // てもさしつかえない: It's okay to (te-form + も + さしつかえ + ない)
        // Examples: 使ってもさしつかえない
        // Kagome uses kana さしつかえ not kanji 差し支え
        (
            GrammarPattern {
                name: "temo_sashitsukaenai",
                tokens: vec![
                    TokenMatcher::Surface("て"),
                    TokenMatcher::Surface("も"),
                    TokenMatcher::Surface("さしつかえ"),
                    TokenMatcher::Surface("ない"),
                ],
                priority: 9,
            },
            ConjugationPattern::TemoSashitsukaenai,
            "n1",
        ),
        // を禁じ得ない: Cannot help but (noun + を + 禁じ + 得 + ない)
        // Examples: 感動を禁じ得ない
        // Kagome uses kanji 得 not hiragana え
        (
            GrammarPattern {
                name: "wo_kinjienai",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("を"),
                    TokenMatcher::Surface("禁じ"),
                    TokenMatcher::Surface("得"),
                    TokenMatcher::Surface("ない"),
                ],
                priority: 10,
            },
            ConjugationPattern::WoKinjienai,
            "n1",
        ),
        // を余儀なくされる: Be forced to (noun + を + 余儀なく + さ + れる)
        // Examples: 中止を余儀なくされる
        (
            GrammarPattern {
                name: "wo_yoginakusareru",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("を"),
                    TokenMatcher::Surface("余儀なく"),
                    TokenMatcher::Surface("さ"),
                    TokenMatcher::Surface("れる"),
                ],
                priority: 10,
            },
            ConjugationPattern::WoYoginakusareru,
            "n1",
        ),
        // てからというもの: Since/ever since (te-form + から + という + もの)
        // Examples: 会ってからというもの幸せだ
        // Kagome lexicalizes という as single particle token
        (
            GrammarPattern {
                name: "te_karatoiumono",
                tokens: vec![
                    TokenMatcher::Surface("て"),
                    TokenMatcher::Surface("から"),
                    TokenMatcher::Surface("という"),
                    TokenMatcher::Surface("もの"),
                ],
                priority: 9,
            },
            ConjugationPattern::TeKaratoiumono,
            "n1",
        ),
        // にもまして: More than/even more (noun + に + も + まして)
        // Examples: 前にもまして元気だ
        // Kagome lexicalizes まして as single adverb token
        (
            GrammarPattern {
                name: "nimo_mashite",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("に"),
                    TokenMatcher::Surface("も"),
                    TokenMatcher::Surface("まして"),
                ],
                priority: 9,
            },
            ConjugationPattern::NimoMashite,
            "n1",
        ),
        // にひきかえ: In contrast to (noun + に + ひきかえ)
        // Examples: 兄にひきかえ弟は静かだ
        // Kagome tokenizes ひきかえ as verb (連用形 of ひきかえる)
        (
            GrammarPattern {
                name: "ni_hikikae",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("に"),
                    TokenMatcher::Surface("ひきかえ"),
                ],
                priority: 7,
            },
            ConjugationPattern::NiHikikae,
            "n1",
        ),
        // Phase 5: Evaluative/Emphatic Patterns (12 patterns)
        // いかん: Depending on (noun + いかん)
        // Examples: 結果いかんで決まる
        (
            GrammarPattern {
                name: "ikan",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("いかん")],
                priority: 6,
            },
            ConjugationPattern::Ikan,
            "n1",
        ),
        // たりとも: Even/not even (noun + たり + と + も)
        // Examples: 一人たりとも許さない
        // Kagome splits as: たり (助動詞) + と + も
        (
            GrammarPattern {
                name: "taritomo",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("たり"),
                    TokenMatcher::Surface("と"),
                    TokenMatcher::Surface("も"),
                ],
                priority: 9,
            },
            ConjugationPattern::Taritomo,
            "n1",
        ),
        // きらいがある: Tend to/have tendency (verb stem + きらい + が + ある)
        // Examples: 遅れるきらいがある
        (
            GrammarPattern {
                name: "kirai_ga_aru",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("きらい"),
                    TokenMatcher::Surface("が"),
                    TokenMatcher::Surface("ある"),
                ],
                priority: 9,
            },
            ConjugationPattern::KiraiGaAru,
            "n1",
        ),
        // しまつだ: End up/come to (verb stem + 始末 + だ/だっ)
        // Examples: 泣く始末だった
        // Pattern for basic copula form
        (
            GrammarPattern {
                name: "shimatsu_da",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("始末"),
                    TokenMatcher::Surface("だ"),
                ],
                priority: 8,
            },
            ConjugationPattern::ShimatsuDa,
            "n1",
        ),
        // Pattern for past tense copula form (だった → だっ + た)
        (
            GrammarPattern {
                name: "shimatsu_datta",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("始末"),
                    TokenMatcher::Surface("だっ"),
                ],
                priority: 8,
            },
            ConjugationPattern::ShimatsuDa,
            "n1",
        ),
        // 割りに（は）: Considering/for (noun/verb + 割り + に)
        // Examples: 安い割りに良い
        (
            GrammarPattern {
                name: "warini",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("割り"),
                    TokenMatcher::Surface("に"),
                ],
                priority: 7,
            },
            ConjugationPattern::Warini,
            "n1",
        ),
        (
            GrammarPattern {
                name: "wariniha",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("割り"),
                    TokenMatcher::Surface("に"),
                    TokenMatcher::Surface("は"),
                ],
                priority: 8,
            },
            ConjugationPattern::Warini,
            "n1",
        ),
        // かいもなく: Despite efforts/in vain (verb stem + 甲斐 + も + なく)
        // Examples: 努力の甲斐もなく失敗した
        (
            GrammarPattern {
                name: "kai_mo_naku",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("甲斐"),
                    TokenMatcher::Surface("も"),
                    TokenMatcher::Surface("なく"),
                ],
                priority: 9,
            },
            ConjugationPattern::KaiMoNaku,
            "n1",
        ),
        // だけまし: At least/better than (noun/verb + だけ + まし)
        // Examples: 怪我がないだけましだ
        (
            GrammarPattern {
                name: "dake_mashi",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("だけ"),
                    TokenMatcher::Surface("まし"),
                ],
                priority: 7,
            },
            ConjugationPattern::DakeMashi,
            "n1",
        ),
        // ないではすまない: Cannot get away with (negative + で + は + すま + ない)
        // Examples: 謝らないではすまない
        (
            GrammarPattern {
                name: "naide_wa_sumanai",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("ない"),
                    TokenMatcher::Surface("で"),
                    TokenMatcher::Surface("は"),
                    TokenMatcher::Surface("すま"),
                    TokenMatcher::Surface("ない"),
                ],
                priority: 11,
            },
            ConjugationPattern::NaideWaSumanai,
            "n1",
        ),
        // ことなしに: Without (verb dictionary + こと + なし + に)
        // Examples: 休むことなしに働く
        (
            GrammarPattern {
                name: "koto_nashini",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("こと"),
                    TokenMatcher::Surface("なし"),
                    TokenMatcher::Surface("に"),
                ],
                priority: 8,
            },
            ConjugationPattern::KotoNashini,
            "n1",
        ),
        // （で）すら: Even (noun + すら / noun + で + すら)
        // Examples: 子供ですら分かる
        (
            GrammarPattern {
                name: "de_sura",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("で"),
                    TokenMatcher::Surface("すら"),
                ],
                priority: 7,
            },
            ConjugationPattern::DeSura,
            "n1",
        ),
        (
            GrammarPattern {
                name: "sura",
                tokens: vec![TokenMatcher::Any, TokenMatcher::Surface("すら")],
                priority: 6,
            },
            ConjugationPattern::DeSura,
            "n1",
        ),
        // ながらに: While remaining/as (multiple forms)
        // This pattern appears in both lexicalized and split forms

        // 1. Lexicalized: 生まれながら + に
        // Kagome lexicalizes 生まれながら as single adverb token
        (
            GrammarPattern {
                name: "nagarani_umare",
                tokens: vec![
                    TokenMatcher::Surface("生まれながら"),
                    TokenMatcher::Surface("に"),
                ],
                priority: 9,
            },
            ConjugationPattern::Nagarani,
            "n1",
        ),
        // 2. Lexicalized: 生まれながら + にして (に + し + て)
        (
            GrammarPattern {
                name: "nagarani_umare_shite",
                tokens: vec![
                    TokenMatcher::Surface("生まれながら"),
                    TokenMatcher::Surface("に"),
                    TokenMatcher::Surface("し"),
                    TokenMatcher::Surface("て"),
                ],
                priority: 10,
            },
            ConjugationPattern::Nagarani,
            "n1",
        ),
        // 3. Split form: Any + ながら + に
        // Examples: 涙ながらに (tears + ながら + に)
        (
            GrammarPattern {
                name: "nagarani_split",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("ながら"),
                    TokenMatcher::Surface("に"),
                ],
                priority: 7,
            },
            ConjugationPattern::Nagarani,
            "n1",
        ),
        // 4. Split form: Any + ながら + にして (に + し + て)
        // Examples: 子供ながらにして, 居ながらにして, 生きながらにして
        (
            GrammarPattern {
                name: "nagarani_shite",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("ながら"),
                    TokenMatcher::Surface("に"),
                    TokenMatcher::Surface("し"),
                    TokenMatcher::Surface("て"),
                ],
                priority: 8,
            },
            ConjugationPattern::Nagarani,
            "n1",
        ),
        // はおろか: Let alone/not to mention (noun + は + おろか)
        // Examples: 英語はおろか日本語も話せない
        (
            GrammarPattern {
                name: "ha_oroka",
                tokens: vec![
                    TokenMatcher::Any,
                    TokenMatcher::Surface("は"),
                    TokenMatcher::Surface("おろか"),
                ],
                priority: 7,
            },
            ConjugationPattern::HaOroka,
            "n1",
        ),
    ]
}
