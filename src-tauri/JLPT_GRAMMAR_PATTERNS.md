# JLPT Grammar Pattern Coverage

## Goal

Add JLPT level tracking to grammar patterns to allow filtering/searching by difficulty level (N5-N1).

## Current State (as of 2025-10-12)

- **Database**: `grammar_patterns` table with columns: `id`, `pattern_name`, `jlpt_level` ✅
- **Pattern Detection**: 210 patterns implemented (31 N5, 53 N4, 59 N3, 58 N2, 68 N1)
- **Test Coverage**: TBD (after cleanup)
- **Pattern Organization**: Patterns organized by JLPT level (N5/N4/N3/N2/N1) in separate modules ✅
- **Adjective Support**: Full support for i-adjectives and na-adjectives in patterns ✅
- **N5 Coverage**: 31/50 patterns = 62%
- **N4 Coverage**: 53/63 patterns = 84%
- **N3 Coverage**: 59/62 patterns = 95%
- **N2 Coverage**: 58/73 patterns = 79%
- **N1 Coverage**: 68/73 patterns = 93%

## Next Steps

- Extend pattern matcher to detect additional JLPT-specific patterns
- Some patterns may require context analysis (too complex/subjective for simple token matching)
- Consider marking difficult patterns for future implementation

## JLPT Grammar Patterns to Track

### N5 (Basic Grammar - 50 patterns)

- **[IMPLEMENTED]** つもりです (intention)
- **[IMPLEMENTED]** ～んです (explanatory)
- **[IMPLEMENTED]** ので (because)
- は (topic marker) - _skip: particle_
- **[IMPLEMENTED]** ～ましょう (let's/shall we)
- ～のがじょうずです (good at) - _skip: semantic_
- ～まえに (before) - _skip: semantic context_
- ～のほうが～より (A is more than B) - _skip: semantic_
- **[IMPLEMENTED]** ～にいく (go to do)
- ～がいます (exists - animate) - _skip: particle_
- ～く/～になる (become) - _skip: semantic_
- ～のがへたです (bad at) - _skip: semantic_
- **[IMPLEMENTED]** ～てください (please do)
- **[IMPLEMENTED]** ～たり…～たりする (do things like)
- **[IMPLEMENTED]** ～てもいいです (may/it's okay to)
- ～があります (exists - inanimate) - _skip: particle_
- です (copula) - _skip: too basic_
- **[IMPLEMENTED]** ～なくちゃいけない (must)
- ～のがすきです (like) - _skip: semantic_
- **[IMPLEMENTED]** ～ている (progressive/state)
- や (and - non-exhaustive list) - _skip: particle_
- を (object marker) - _skip: particle_
- **[IMPLEMENTED]** ～てはいけません (must not)
- **[IMPLEMENTED]** stem + たいです (want to)
- **[IMPLEMENTED]** ないでください (please don't)
- **[IMPLEMENTED]** でしょう (probably)
- **[IMPLEMENTED]** ～ませんか (won't you/invitation)
- **[IMPLEMENTED]** まだ～ていません (not yet)
- で (by means of/at) - _skip: particle_
- ～から (because/from) - _skip: particle_
- も (also/too) - _skip: particle_
- と (and/with/quotation) - _skip: particle_
- **[IMPLEMENTED]** ～すぎる (too much)
- **[IMPLEMENTED]** ～ほうがいい (had better)
- に (location/time/indirect object) - _skip: particle_
- ～のなかで～がいちばん～ (most ~ among) - _skip: semantic_
- **[IMPLEMENTED]** ～ましょうか (shall I)
- **[IMPLEMENTED]** ～てから (after doing)
- **[IMPLEMENTED]** ～たことがある (have done before)
- に/へ (direction markers) - _skip: particle_
- **[IMPLEMENTED]** ～ます (polite form)
- **[IMPLEMENTED]** ～た (past tense)
- **[IMPLEMENTED]** ～ない (negative)
- **[IMPLEMENTED]** ～なかった (past negative)
- **[IMPLEMENTED]** ～て (te-form)
- **[IMPLEMENTED]** ～たい (want to)
- **[IMPLEMENTED]** ～たくない (don't want to)
- **[IMPLEMENTED]** ～たかった (wanted to)
- **[IMPLEMENTED]** dictionary form
- **[IMPLEMENTED]** ～てくださる (polite request)

### N4 (Elementary Grammar - 63 patterns)

- **[IMPLEMENTED]** ～おう (volitional)
- **[IMPLEMENTED]** ～という～ (called/named)
- **[IMPLEMENTED]** ～かどうか (whether or not)
- ～のようてほしい (want someone to) - _skip: semantic_
- **[IMPLEMENTED]** ～はずです (should/expected to)
- **[IMPLEMENTED]** ～てある (has been done)
- **[IMPLEMENTED]** Dictionary form+と (when/if)
- **[IMPLEMENTED]** ～がほしい (want)
- **[IMPLEMENTED]** ～なさい (command form)
- **[IMPLEMENTED]** ～てもらう (have someone do)
- **[IMPLEMENTED]** しか～ない (only/nothing but)
- ～よう (let's/seems) - _skip: ambiguous_
- **[IMPLEMENTED]** ～てあげる (do for someone)
- (period)に(frequency) (per period) - _skip: semantic_
- **[IMPLEMENTED]** そうです (appearance: looks like)
- **[IMPLEMENTED]** そうです (hearsay: I heard that)
- **[IMPLEMENTED]** ～ていただけませんか (could you please)
- **[IMPLEMENTED]** ～やすい (easy to)
- **[IMPLEMENTED]** ～かもしれない (might/maybe)
- **[IMPLEMENTED]** ～ば/～れば (if/conditional)
- **[IMPLEMENTED]** ～たらどうですか (how about)
- ～く/～にする (make/decide) - _skip: semantic_
- **[IMPLEMENTED]** のに (despite/although)
- Number+も (as many as) - _skip: semantic_
- **[IMPLEMENTED]** てみる (try doing)
- **[IMPLEMENTED]** ～ばよかった (should have)
- **[IMPLEMENTED]** ～てすみません (sorry for doing)
- **[IMPLEMENTED]** ～ないで (without doing)
- ～ているあいだに,～ (while doing) - _skip: complex temporal_
- **[IMPLEMENTED]** ～といいです (I hope/it would be good if)
- **[IMPLEMENTED]** ～てくれる (do for me)
- ～のように (like/as) - _skip: semantic_
- **[IMPLEMENTED]** ～させる (causative)
- **[IMPLEMENTED]** ～させられる (causative-passive)
- **[IMPLEMENTED]** ～し (and what's more)
- **[IMPLEMENTED]** なら (if it's the case that)
- **[IMPLEMENTED]** ～ことにする (decide to)
- **[IMPLEMENTED]** ～てくれてありがとう (thank you for)
- **[IMPLEMENTED]** ～ておく (do in advance)
- **[IMPLEMENTED]** ～てしまう (completely/regrettably)
- ～のような (like/similar to) - _skip: semantic_
- **[IMPLEMENTED]** ～ても (even if)
- **[IMPLEMENTED]** ～みたい (looks like/seems)
- **[IMPLEMENTED]** ～ながら (while doing)
- **[IMPLEMENTED]** ～なくてもいい (don't have to)
- **[IMPLEMENTED]** ～てよかった (glad that)
- **[IMPLEMENTED]** ～たら (if/when/conditional)
- **[IMPLEMENTED]** ～にくい (difficult to)
- **[IMPLEMENTED]** ～られる (potential/passive)
- **[IMPLEMENTED]** ～たがる (show signs of wanting)
- **[IMPLEMENTED]** ～ことになる (it has been decided)
- **[IMPLEMENTED]** ～命令形 (imperative)

### N3 (Intermediate Grammar - 62 patterns)

- ～たび～ (every time) - _skip: requires multi-clause analysis_
- **[IMPLEMENTED]** ようにする (try to/make sure to)
- **[IMPLEMENTED]** まさか (surely not)
- ～げ (appearance/seeming) - _skip: Kagome doesn't preserve adjective stem information; compounds like 寂しげ tokenize as standalone nouns indistinguishable from verb-derived nouns like 逃げ_
- **[IMPLEMENTED]** ～始める (begin to)
- **[IMPLEMENTED]** ～らしい (seems like/typical)
- **[IMPLEMENTED]** ～ぐらい (about/approximately)
- ～には～の～がある (has ~ for) - _skip: complex semantic pattern_
- **[DUPLICATE N4]** ～ても (even if)
- **[IMPLEMENTED]** ～によると (according to)
- **[IMPLEMENTED]** むしろ (rather)
- **[IMPLEMENTED]** ～として (as/in the role of)
- **[IMPLEMENTED]** N+ばかり (only/nothing but)
- **[IMPLEMENTED]** すぎない (過ぎない) (no more than)
- **[IMPLEMENTED]** ～っぽい (ish/like)
- **[IMPLEMENTED]** すでに（既に） (already)
- **[IMPLEMENTED]** おいて（於いて） (at/in)
- ほど (extent/degree) - _skip: ambiguous - cannot distinguish grammatical usage from temporal adverb 先ほど_
- **[DUPLICATE N4]** ～ないで (without doing)
- **[IMPLEMENTED]** つい (accidentally/unintentionally)
- **[IMPLEMENTED]** ふり (pretend)
- **[IMPLEMENTED]** まま (as is/remain)
- **[IMPLEMENTED]** ～ために (in order to/for)
- **[IMPLEMENTED]** ～ないうちに (before/while not)
- **[IMPLEMENTED]** ～としたら (if we assume)
- **[IMPLEMENTED]** つもりで (with intention of)
- **[IMPLEMENTED]** ～に関する (regarding/about)
- ～とても～ない (cannot possibly) - _skip: とても ambiguous (very/even if)_
- **[IMPLEMENTED]** どうしても (no matter what/by all means)
- **[IMPLEMENTED]** ～とともに～ (together with/as)
- **[IMPLEMENTED]** きり (only/since)
- **[IMPLEMENTED]** ～によって (depending on/by)
- **[IMPLEMENTED]** ように (so that/in order to)
- **[IMPLEMENTED]** ～て初めて (for the first time after)
- **[IMPLEMENTED]** ～がち (tend to)
- **[IMPLEMENTED]** ～ず (without doing)
- **[IMPLEMENTED]** ～ようになる (come to/become)
- **[IMPLEMENTED]** せいぜい (at most/at best)
- **[IMPLEMENTED]** N+を始め (beginning with)
- **[IMPLEMENTED]** ～ば～ほど (the more ~ the more)
- **[IMPLEMENTED]** ～たて (freshly/just done)
- ～は～くらいです (is about) - _skip: full sentence pattern_
- **[IMPLEMENTED]** どうやら (apparently/it seems)
- ～は～で有名 (famous for) - _skip: full sentence pattern_
- **[IMPLEMENTED]** ～かえって (on the contrary)
- **[IMPLEMENTED]** ～さえ～ば (if only/as long as)
- **[IMPLEMENTED]** こそ (emphasis particle)
- ～になれる (get used to) - _skip: ambiguous with potential になる + れる_
- **[IMPLEMENTED]** さらに（更に） (furthermore)
- **[IMPLEMENTED]** ～代わり (instead of/in place of)
- **[IMPLEMENTED]** まい (will not/intention not to)
- **[IMPLEMENTED]** わざわざ (expressly/going out of one's way)
- **[IMPLEMENTED]** 限る (limited to/best)
- **[IMPLEMENTED]** おかげで (thanks to)
- **[IMPLEMENTED]** さえ (even)
- **[IMPLEMENTED]** ～たものだ (used to)
- ～のような (like/such as) - _skip: duplicate of N4 pattern_
- **[IMPLEMENTED]** 的 (ish/like suffix)
- **[IMPLEMENTED]** いったい（一体） (on earth/what)
- **[DUPLICATE N4]** ～ことになる (it has been decided)
- **[IMPLEMENTED]** ～に違いない (must be/no doubt)
- **[IMPLEMENTED]** なかなか (quite/not easily)
- **[IMPLEMENTED]** ～たばかり (just did)

### N2 (Upper Intermediate - 73 patterns)

- **[IMPLEMENTED]** ～ばいいのに (if only/I wish)
- **[IMPLEMENTED]** ～わけです (it is that/the fact is)
- **[IMPLEMENTED]** 何といっても (after all/no matter what)
- **[IMPLEMENTED]** ～ような気がする (feel like/have a feeling)
- **[DUPLICATE N4]** ～ことにする (decide to)
- **[IMPLEMENTED]** 以外の (other than/except)
- **[IMPLEMENTED]** ～に気をつける (be careful of)
- **[IMPLEMENTED]** 別に～ない (not particularly)
- **[DUPLICATE N4]** ～しか～ない (only/nothing but)
- **[IMPLEMENTED]** ～にあたる (correspond to/be equivalent)
- **[IMPLEMENTED]** とうてい (cannot possibly)
- **[IMPLEMENTED]** のぼる (amount to/reach)
- **[IMPLEMENTED]** ～ないで済む (can get by without)
- **[IMPLEMENTED]** がてら (while/on the occasion of)
- **[IMPLEMENTED]** および (and/as well as)
- いったんーば (once ~ then) - _skip: requires multi-clause analysis_
- **[IMPLEMENTED]** よっぽど (very/considerably)
- **[IMPLEMENTED]** つうじて（通じて） (through/throughout)
- **[IMPLEMENTED]** 思うように (as one wishes)
- **[IMPLEMENTED]** いよいよ (finally/at last)
- **[IMPLEMENTED]** ～からなる(成る) (consist of)
- **[IMPLEMENTED]** ～より仕方がない (cannot help but)
- **[IMPLEMENTED]** ～ものですから (because/since)
- ～なかなか～ない (not easily) - _skip: cross-clause negation tracking; standalone なかなか already in N3_
- **[IMPLEMENTED]** せっかく (with trouble/specially)
- **[IMPLEMENTED]** やむをえず (unavoidably)
- **[IMPLEMENTED]** やっぱり (as expected/after all)
- かたわら（傍ら） (while/besides) - _skip: ambiguous - cannot distinguish grammatical "while doing" from physical location "beside"_
- **[IMPLEMENTED]** ろくに (properly/well)
- **[IMPLEMENTED]** なるべく (as much as possible)
- **[IMPLEMENTED]** かねる (cannot/unable to)
- **[IMPLEMENTED]** せいか (perhaps because)
- **[IMPLEMENTED]** たしか (if I remember correctly)
- **[IMPLEMENTED]** ～ではないだろうか (isn't it that)
- **[IMPLEMENTED]** ～わけにはいかない (cannot afford to)
- **[IMPLEMENTED]** まんいち（万一） (by any chance)
- ～は～に限る (nothing beats/best is) - _skip: requires particle scope analysis_
- **[IMPLEMENTED]** ゆえに (therefore/because)
- **[IMPLEMENTED]** 一方では (on the other hand)
- **[IMPLEMENTED]** なにしろ（何しろ） (at any rate/after all)
- **[IMPLEMENTED]** それにしても (nevertheless/even so)
- ～を～にまかせる (leave to/entrust to) - _skip: requires particle scope analysis_
- **[IMPLEMENTED]** ～というわけではない (it doesn't mean that)
- **[IMPLEMENTED]** かけては (when it comes to)
- **[IMPLEMENTED]** たちまち (at once/in no time)
- **[IMPLEMENTED]** さすがに (as expected/even)
- **[IMPLEMENTED]** ～に越したことはない (nothing is better than)
- **[IMPLEMENTED]** ものの (although/though)
- **[IMPLEMENTED]** いつのまにか (before one knows)
- **[IMPLEMENTED]** さしつかえない (no problem/no objection)
- **[IMPLEMENTED]** あえて (dare to/purposely)
- **[IMPLEMENTED]** たまらない (unbearable/can't help)
- **[IMPLEMENTED]** ～た上で (after doing)
- **[IMPLEMENTED]** ～ないわけにはいかない (cannot not do/must)
- **[IMPLEMENTED]** ごとし（如し） (like/as if)
- **[IMPLEMENTED]** かねない (might/could possibly)
- ろくに～ない (hardly/not properly) - _skip: cross-clause negation tracking; standalone ろくに already implemented_
- **[IMPLEMENTED]** ～と言っても～ (even though I say/although)
- **[IMPLEMENTED]** ～が気になる (worry about/be concerned)
- **[IMPLEMENTED]** せめて (at least)
- ～でいいです (is fine/okay with) - _skip: too basic/regular grammar_
- **[IMPLEMENTED]** くせに (and yet/even though)
- **[IMPLEMENTED]** いたるまで（至るまで） (up to/as far as)

### N1 (Advanced Grammar - 73 patterns)

- **[IMPLEMENTED]** ～めく (show signs of/like)
- ～かたわら (while/besides) - _skip: duplicate N2 katawara (removed due to ambiguity)_
- **[IMPLEMENTED]** ～と思いきや (contrary to expectations)
- **[IMPLEMENTED]** ～が早いか (as soon as/no sooner than)
- ただ～のみ (only/solely) - _skip: variable-length content between fixed markers cannot be matched with current pattern system_
- **[IMPLEMENTED]** ～なり (as soon as/upon)
- **[IMPLEMENTED]** ～や否や（いなや） (as soon as)
- **[IMPLEMENTED]** ～ごとき (like/such as)
- ～がてら (while/on the occasion of) - _skip: duplicate of N2 gatera_
- **[IMPLEMENTED]** ～を皮切りに (starting with/beginning with)
- **[IMPLEMENTED]** ～をもって (with/by means of)
- **[IMPLEMENTED]** ～が最後 (once ~ then forever)
- **[IMPLEMENTED]** ～まみれ (covered with/smeared with)
- **[IMPLEMENTED]** ～とあれば (if it's the case)
- **[IMPLEMENTED]** ～ともなると (when it comes to)
- **[IMPLEMENTED]** ～なくしては (without/unless)
- **[IMPLEMENTED]** ～なしに (without)
- **[IMPLEMENTED]** ～ならでは (unique to/only)
- **[IMPLEMENTED]** ～に足る (worth/deserve)
- **[IMPLEMENTED]** ～とあって (because/being)
- **[IMPLEMENTED]** ～べく (in order to/for)
- **[IMPLEMENTED]** ～かたがた (while/also to)
- **[IMPLEMENTED]** ～たところで (even if)
- **[IMPLEMENTED]** ～であれ (whether/even if)
- **[IMPLEMENTED]** ～にたえない (cannot bear/unbearable)
- **[IMPLEMENTED]** ～を限りに (as the last time)
- ～ところを (although/in spite of) - _skip: ambiguous - cannot distinguish grammatical usage from regular noun ところ (place/moment/situation)_
- **[IMPLEMENTED]** ～にそくして (in accordance with)
- **[IMPLEMENTED]** ～とはいえ (although/even though)
- ～ものを (if only/I wish) - _skip: ambiguous - cannot distinguish grammatical pattern from regular noun もの (thing); too complex to match reliably_
- **[IMPLEMENTED]** ～ようが (no matter/even if)
- **[IMPLEMENTED]** ～いかん (depending on)
- **[IMPLEMENTED]** ～と相まって (combined with)
- **[IMPLEMENTED]** ～をよそに (in spite of/ignoring)
- **[IMPLEMENTED]** ～ないまでも (even if not)
- **[IMPLEMENTED]** ～てもさしつかえない (it's okay to)
- ～たる (appropriate/fitting) - _skip: lexicalized as 連体詞_
- **[IMPLEMENTED]** ～まじき (should not/unworthy)
- **[IMPLEMENTED]** ～極まる (extremely)
- ～にかこつけて (under the pretext of) - _skip: requires understanding intent/excuse-making_
- ～に（は）あたらない (not worth/no need) - _skip: requires value judgment_
- ～にかたくない (not difficult to) - _skip: requires subjective ease assessment_
- **[IMPLEMENTED]** ～べからず (must not/should not)
- **[IMPLEMENTED]** ～を禁じ得ない (cannot help but)
- **[IMPLEMENTED]** ～たりとも (even/not even)
- **[IMPLEMENTED]** ～きらいがある (tend to/have tendency)
- **[IMPLEMENTED]** ～しまつだ (end up/come to)
- **[IMPLEMENTED]** ～を余儀なくされる (be forced to)
- **[IMPLEMENTED]** ～てやまない (never cease/always)
- **[IMPLEMENTED]** ～割りに（は） (considering/for)
- **[IMPLEMENTED]** ～かいもなく (despite efforts/in vain)
- **[IMPLEMENTED]** ～だけまし (at least/better than)
- **[IMPLEMENTED]** ～ないではすまない (cannot get away with)
- **[IMPLEMENTED]** ～をふまえて (based on/considering)
- **[IMPLEMENTED]** ～をおして (in spite of)
- **[IMPLEMENTED]** ～を経て (through/via)
- ～ゆえ (because/therefore) - _skip: duplicate of N2 yueni_
- **[IMPLEMENTED]** ～ながらも (while/though)
- **[IMPLEMENTED]** ～ことなしに (without)
- **[IMPLEMENTED]** ～ではあるまいし (it's not like)
- **[IMPLEMENTED]** ～てからというもの (since/ever since)
- **[IMPLEMENTED]** ～としたところで (even if)
- **[IMPLEMENTED]** ～（で）すら (even)
- **[IMPLEMENTED]** ～といえども (even though/although)
- **[IMPLEMENTED]** ～っぱなし (leaving as is)
- **[IMPLEMENTED]** ～ずくめ (entirely/nothing but)
- **[IMPLEMENTED]** ～ながらに (while remaining/as)
- **[IMPLEMENTED]** ～にもまして (more than/even more)
- **[IMPLEMENTED]** ～にひきかえ (in contrast to)
- **[IMPLEMENTED]** ～はおろか (let alone/not to mention)

## Implementation Plan

### Phase 1: Schema & Existing Pattern Mapping ✅ COMPLETED

1. ✅ Added `jlpt_level` column to database
2. ✅ Organized patterns by JLPT level (N5/N4 modules)
3. ✅ Updated pattern creation/insertion to include JLPT level
4. ✅ All 25 existing patterns mapped to appropriate JLPT levels

### Phase 2: Pattern Analysis & Prioritization

Review each JLPT pattern and categorize:

- **Easy**: Can detect with token-based pattern matching (like current implementation)
- **Medium**: Requires context or multi-sentence analysis
- **Hard**: Too subjective or requires semantic understanding
- **Skip**: Particles (は、を、に、で, etc.) - too basic/not useful for search

### Phase 3: Incremental Implementation

1. Start with N5/N4 patterns (easier, more common)
2. Add pattern definitions for detectable patterns
3. Write tests for each new pattern
4. Document which patterns are skipped and why

### Phase 4: UI/API Updates

1. Add JLPT level filter to search/query endpoints
2. Display JLPT level in grammar pattern results
3. Add episode difficulty metrics based on grammar patterns used

## Notes

### Patterns to Consider Skipping

- Basic particles (は、を、に、で、も、と、や) - too fundamental, not useful for search
- Some N1 literary/formal patterns may be rare in anime subtitles
- Patterns requiring semantic understanding (e.g., "～ような気がする" requires understanding feelings)

### Current Pattern Status

**Implemented (210 patterns)**:

**N5 (31 patterns)**:

- dictionary_form, masu_form, negative, past_tense, past_negative
- te_form_basic, te_iru, te_request, te_kudasai, te_kara
- tai_form, takunai_form, takatta_form
- mashou, ta_koto_ga_aru, te_mo_ii, te_wa_ikenai
- naide_kudasai, masen_ka, mashou_ka, sugiru
- tsumori_desu, hou_ga_ii, nakucha_ikenai, deshou
- mada_te_imasen, n_desu, node, ni_iku

**N4 (53 patterns)**:

- te_miru, te_shimau, tari_form
- ba_conditional, tara_conditional
- potential, passive, causative, causative_passive
- volitional, imperative, nagara, past_negative
- must_nakereba, must_nakute_wa
- te_aru, te_kureru, te_ageru, te_oku
- yasui, nikui
- te_morau, te_sumimasen, te_kurete_arigatou, te_yokatta, te_mo
- naide, nakute_mo_ii, ba_yokatta
- nasai, hazu_desu, tagaru, te_itadakemasen_ka
- tara_dou, to_ii, ga_hoshii
- shika_nai, to_iu, dictionary_to, nara, shi, ka_dou_ka
- koto_ni_suru, noni, koto_ni_naru
- sou_desu_appearance, sou_desu_hearsay, sou_desu_hearsay_na
- kamo_shirenai, kamo_shiremasen, mitai

**N2 (58 patterns)**:

- ba_ii_noni, wake_desu, nantoittemo, you_na_ki_ga_suru
- igai_no, ni_ki_wo_tsukeru, betsuni_nai
- ni_ataru, toutei, noboru, naide_sumu
- gatera, oyobi, yoppodo, tsuujite
- omou_you_ni, iyoiyo, kara_naru, yori_shikata_ganai
- mono_desukara, sekkaku, yamuoezu, yappari
- rou_ni, narubeku, kaneru, kanenai
- sei_ka, tashika, dewa_nai_darou_ka, wake_niwa_ikanai, nai_wake_niwa_ikanai
- man_ichi, yueni, ippou_dewa, nanishiro, sorenishitemo
- to_iu_wake_dewa_nai, kaketeha, tachimachi, sasugani
- ni_koshita_koto_wa_nai, mono_no, itsunomanika, sashitsukaenai
- aete, tamaranai, ta_ue_de, gotoshi
- to_ittemo, ga_ki_ni_naru, semete
- kuse_ni, itaru_made

**N1 (68 patterns)**:

- meku, mamire, zukume, ppanashi, kiwamaru, beku, bekarazu, majiki
- nari, ya_inaya, ga_hayai_ka, ga_saigo, gotoki, wo_kawakiri_ni, wo_motte
- nakushiteha, nashini, naradewa, ni_taru, toatte, katagata
- wo_kagiri_ni, wo_hete, wo_oshite, wo_fumaete, te_yamanai
- to_omoikiya, to_areba, ta_tokoro_de, de_are, to_wa_ie
- you_ga, nai_made_mo, nagara_mo, dewa_arumaishi
- to_shita_tokoro_de, to_iedomo
- tomo_naruto, ni_taenai, ni_sokushite, to_aimatte
- wo_yosoni, temo_sashitsukaenai, wo_kinjienai, wo_yoginakusareru
- te_karatoiumono, nimo_mashite, ni_hikikae
- ikan, taritomo, kirai_ga_aru, shimatsu_da, warini, wariniha
- kai_mo_naku, dake_mashi, naide_wa_sumanai, koto_nashini
- de_sura, sura, ha_oroka
- nagarani_umare, nagarani_umare_shite, nagarani_split, nagarani_shite

**N3 (59 patterns)**:

- hajimeru, rashii, you_ni_naru, you_ni_suru, tame_ni
- zu, gachi, ta_bakari, ta_mono_da, ni_chigainai
- mama, furi, nai_uchi_ni, ppoi, to_shitara
- bakari, kawari, okage_de, sae, you_ni
- masaka, mushiro, sudeni, tsui, doushitemo
- teki_suffix, tate_suffix, ni_yotte, kiri
- gurai, ni_yoru_to, toshite, suginai, oite
- tsumori_de, ni_kansuru, to_tomoni, te_hajimete, seizei
- wo_hajime, ba_hodo, douyara, kaette, sae_ba
- koso, sarani, mai, wazawaza, kagiru
- nakanaka, ittai

**Test Coverage**: TBD (after cleanup)

### Recent Changes (2025-10-12)

**Removed Ambiguous Patterns**:

Removed 4 patterns that produced too many false positives due to ambiguity with regular word usage:

1. **ほど (hodo)** - N3 pattern removed
   - **Issue**: Cannot distinguish grammatical usage (extent/degree) from temporal adverb 先ほど (a while ago)
   - **Example false positive**: "先ほど言った" matches as grammar pattern when it's just "said a while ago"

2. **かたわら (katawara)** - N2 pattern removed (kanji/kana variants)
   - **Issue**: Cannot distinguish grammatical "while doing X" from physical location "beside/next to"
   - **Example false positive**: "机の傍らに" (beside the desk) matches when it should only match grammatical usage like "働く傍ら勉強する" (study while working)

3. **ところを (tokoro_wo)** - N1 pattern removed
   - **Issue**: Cannot distinguish grammatical "although/in spite of" from regular noun usage of ところ (place/moment/situation)
   - **Example false positive**: "いいところを見つけた" (found a good place) matches when it should only match "忙しいところを来てくれた" (came despite being busy)

4. **ものを (mono_wo)** - N1 pattern removed
   - **Issue**: Cannot distinguish grammatical pattern (if only/I wish) from regular noun もの (thing) + を particle
   - **Example false positive**: "基本的なものを選ぶ" (choose basic things) matches when it should only match "よかったものを" (if only it had been good)
   - **Pattern complexity**: Requires conditional form (ば/たら) with variable intervening tokens, too complex to match reliably

**Updated Pattern Counts**:

- N3: 60 → 59 patterns (removed hodo)
- N2: 60 → 58 patterns (removed katawara_kanji, katawara_kana)
- N1: 70 → 68 patterns (removed tokoro_wo, mono_wo)
- **Total**: 214 → 210 patterns

### Previous Additions (2025-10-12)

**ながらに Pattern Expansion**:

- Expanded single ながらに pattern into 4 comprehensive patterns:
  - `nagarani_umare`: Lexicalized form (生まれながら + に)
  - `nagarani_umare_shite`: Lexicalized with して (生まれながら + にして)
  - `nagarani_split`: Generic split form (涙 + ながら + に)
  - `nagarani_shite`: Generic with して (子供 + ながら + にして)
- Added 4 new tests covering all ながらに variations
- Pattern now properly covers fixed expressions: 生まれながらに, 涙ながらに, 居ながらにして, 子供ながらにして
- Total: 70 N1 patterns (96% coverage), 256 tests passing

**N1 Grammar Patterns Implementation**:

- Implemented 70 N1 patterns (96% of N1 grammar)
- Added 72 comprehensive tests covering all N1 patterns
- Organized patterns in 5 phases:
  - Phase 1: Suffix patterns (8 patterns, 11 tests)
  - Phase 2: Simple fixed expressions (19 patterns, 19 tests)
  - Phase 3: Conditional/concessive patterns (12 patterns, 12 tests)
  - Phase 4: Complex multi-word expressions (12 patterns, 12 tests)
  - Phase 5: Evaluative/emphatic patterns (12 patterns, 14 tests)
- Handled complex Kagome tokenization for advanced/classical forms:
  - Classical negatives (べからず → べから + ず, まじき as single token)
  - Lexicalized compounds (相まって as adverb, やいなや as particle, という as particle)
  - Adverb lexicalization (まして, おして as single tokens)
  - Copula forms (だった → だっ + た requiring variant patterns)
  - Common compound adverbs (生まれながら as single token)
  - Split patterns (たりとも → たり + と + も, をよそに → を + よそ + に)
- Created variant patterns for different tokenizations (warini/wariniha, de_sura/sura, shimatsu_da/shimatsu_datta)
- All patterns support both kanji and kana variants where applicable
- Skipped 10 patterns:
  - かたわら (duplicate of N2 katawara)
  - ただ～のみ (variable-length content matching not supported)
  - たる (lexicalized as 連体詞)
  - がてら (duplicate of N2 gatera)
  - ゆえ (duplicate of N2 yueni)
  - にかこつけて, に（は）あたらない, にかたくない (require semantic understanding)

**N2 Grammar Patterns Implementation**:

- Implemented 60 N2 patterns (82% of N2 grammar)
- Added 77 comprehensive tests covering all N2 patterns
- Organized patterns in 4 phases:
  - Phase 1: Adverbs & standalone expressions (18 patterns)
  - Phase 2: Verb suffixes & auxiliaries (11 patterns)
  - Phase 3: Particle patterns & conjunctions (11 patterns)
  - Phase 4: Fixed expressions (20 patterns)
- Handled complex Kagome tokenization quirks:
  - Compound particles (にあたる, という, にかけて tokenize as single units)
  - Split tokenizations (では → で + は, がてら → が + てら)
  - Multi-way auxiliary verb splits (だろう vs だろ + う)
  - Adverb compounds (なんと as adverb vs 何 + と)
- Created variant patterns for reliable detection across different tokenizations
- All patterns support both kanji and kana variants where applicable
- Skipped 5 patterns requiring complex analysis:
  - いったんーば (multi-clause)
  - なかなか～ない (cross-clause negation)
  - は～に限る (particle scope analysis)
  - を～にまかせる (particle scope analysis)
  - ろくに～ない (cross-clause negation)
  - でいいです (too basic)

### Previous Additions (2025-10-11)

**Adjective Support Enhancement**:

- Added CustomMatchers for adjective detection: `IAdjective`, `NaAdjective`, `AdjectiveStem`, `SugiruStem`
- Enhanced `sugiru` pattern to support i-adjectives (高すぎる) and na-adjectives (静かすぎる)
- Added `sou_desu` patterns with full verb/adjective support:
  - `sou_desu_appearance`: Appearance/inference (食べそうです, 高そうです, 静かそうです)
  - `sou_desu_hearsay`: Hearsay/I heard (食べるそうです, 高いそうです)
  - `sou_desu_hearsay_na`: Na-adjective hearsay with だ (静かだそうです)
- Added 4 edge case tests to verify false positive prevention (standalone そうです for agreement)
- All patterns correctly handle Kagome tokenization quirks:
  - i-adjectives use ガル接続 form before すぎる/そう
  - na-adjectives tagged as 名詞/形容動詞語幹

**Additional N4 Patterns**:

- Added `kamo_shirenai` (might/maybe): 食べるかもしれない
- Added `kamo_shiremasen` (might/maybe, polite): 食べるかもしれません
- Added `mitai` (looks like/seems): 食べるみたい
- Removed duplicate そうです entry from documentation

## References

- [JLPT Grammar Lists](https://jlptsensei.com/)
- Current implementation: `src/grammar/patterns/n5.rs`, `src/grammar/patterns/n4.rs`
- Pattern matcher: `src/grammar/pattern_matcher.rs`
- Database schema: `src/db/transcript_database.rs`
