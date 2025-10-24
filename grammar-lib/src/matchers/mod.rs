// Consolidated matcher modules organized by JLPT level
mod common_matchers;
mod n2_matchers;
mod n3_matchers;
mod n4_matchers;
mod n5_matchers;

use common_matchers::*;
use n2_matchers::*;
use n3_matchers::*;
use n4_matchers::*;
use n5_matchers::*;

use crate::types::KagomeToken;
use std::fmt::Debug;

/// Trait for token matching logic
///
/// Each custom matcher implements this trait to provide matching logic for a single token
pub trait TokenMatcherLogic: Debug + Send + Sync {
    /// Check if the token matches this matcher's criteria
    fn matches(&self, token: &KagomeToken) -> bool;
}

/// Predefined custom matchers to avoid function pointer comparison issues
#[derive(Debug, Clone, PartialEq)]
pub enum CustomMatcher {
    /// Match たい as auxiliary verb or adjective
    TaiForm,
    /// Match たく from たい
    TakuForm,
    /// Match たかっ from たい
    TakattaForm,
    /// Match なかっ from ない
    NakattaForm,
    /// Match なけれ from ない
    NakereForm,
    /// Match なく from ない
    NakuForm,
    /// Match させ from させる
    SaseForm,
    /// Match ichidan verb in 未然形
    IchidanMizen,
    /// Match godan verb in 未然形
    GodanMizen,
    /// Match られる or れる (ichidan potential/passive)
    RareruForm,
    /// Match れる (godan passive)
    ReruForm,
    /// Match える (godan potential)
    EruForm,
    /// Match させる or せる (causative)
    CausativeForm,
    /// Match たら (conditional)
    TaraForm,
    /// Match だ or た (past auxiliary)
    PastAuxiliary,
    /// Match て or で as either surface or particle
    TeDeForm,
    /// Match verb in 連用形 or 連用タ接続 (for flexible patterns)
    FlexibleVerbForm,
    /// Match specific verb forms for must patterns
    MustPattern,
    /// Match 未然形 verbs that are NOT potential forms (excludes れる/られる)
    NonPotentialMizen,
    /// Match 未然形 verbs excluding なる (for shika_nai pattern)
    NonNaruMizen,
    /// Match いい or 良い (good/okay)
    IiForm,
    /// Match いけない or いけません (must not)
    IkenaiForm,
    /// Match ましょ + う (let's/shall we)
    MashouForm,
    /// Match ませ + ん (polite negative)
    MasenForm,
    /// Match まし (polite past stem)
    MashiForm,
    /// Match でし (copula past stem)
    DeshiForm,
    /// Match imperative forms (命令形, 命令ｒｏ, 命令ｉ)
    ImperativeForm,
    /// Match たり particle (並立助詞)
    TariParticle,
    /// Match でしょう (probably)
    DeshouForm,
    /// Match ん or の before です (explanatory)
    NDesuForm,
    /// Match よかった (was good/glad)
    YokattaForm,
    /// Match た from たい + がる
    TagaruForm,
    /// Match いい in といい context (can be verb いう or adj いい)
    ToIiForm,
    /// Match し particle (接続助詞)
    ShiParticle,
    /// Match verb 連用形 or adjective stem (for すぎる and そう appearance patterns)
    SugiruStem,
    /// Match plain form verb or adjective (for そうだ hearsay)
    SouHearsayStem,
    /// Match まい (negative volition)
    MaiForm,
    /// Match っぽい suffix
    PpoiForm,
    /// Match 的 suffix
    TekiSuffix,
    /// Match たて suffix
    TateSuffix,
    /// Match ぐらい or くらい (about/approximately)
    GuraiForm,
    /// Match おいて or において (at/in)
    OiteForm,
    /// Match に関する or に関して (regarding/about)
    NiKansuruForm,
    /// Match 初めて as adverb (for て初めて pattern)
    HajimeteAdverb,
    /// Match および or 及び as conjunction (not verb)
    OyobiConjunction,
    /// Match noun (名詞)
    Noun,
    /// Match dependent noun もの (名詞/非自立) for mono_no pattern
    DependentNounMono,
    /// Match verbs where base_form ends with a specific suffix (e.g., base_form ends with "めく")
    VerbWithBaseSuffix(&'static str),
    /// Match nouns where base_form ends with a specific suffix (e.g., base_form ends with "まみれ")
    NounWithBaseSuffix(&'static str),
    /// Match words that can precede でしょう (verbs, adjectives, nouns, auxiliaries)
    DeshouPreceding,
    /// Match ichidan verb following が particle (potential construction)
    GaPotentialVerb,
    /// Match any particle (助詞)
    Particle,
}

/// Centralized matching logic for all custom matchers
pub fn matches(matcher: &CustomMatcher, token: &KagomeToken) -> bool {
    match matcher {
        CustomMatcher::TaiForm => TaiFormMatcher.matches(token),
        CustomMatcher::TakuForm => TakuFormMatcher.matches(token),
        CustomMatcher::TakattaForm => TakattaFormMatcher.matches(token),
        CustomMatcher::NakattaForm => NakattaFormMatcher.matches(token),
        CustomMatcher::NakereForm => NakereFormMatcher.matches(token),
        CustomMatcher::NakuForm => NakuFormMatcher.matches(token),
        CustomMatcher::SaseForm => SaseFormMatcher.matches(token),
        CustomMatcher::IchidanMizen => IchidanMizenMatcher.matches(token),
        CustomMatcher::GodanMizen => GodanMizenMatcher.matches(token),
        CustomMatcher::RareruForm => RareruFormMatcher.matches(token),
        CustomMatcher::ReruForm => ReruFormMatcher.matches(token),
        CustomMatcher::EruForm => EruFormMatcher.matches(token),
        CustomMatcher::CausativeForm => CausativeFormMatcher.matches(token),
        CustomMatcher::TaraForm => TaraFormMatcher.matches(token),
        CustomMatcher::PastAuxiliary => PastAuxiliaryMatcher.matches(token),
        CustomMatcher::TeDeForm => TeDeFormMatcher.matches(token),
        CustomMatcher::FlexibleVerbForm => FlexibleVerbFormMatcher.matches(token),
        CustomMatcher::MustPattern => MustPatternMatcher.matches(token),
        CustomMatcher::NonPotentialMizen => NonPotentialMizenMatcher.matches(token),
        CustomMatcher::NonNaruMizen => NonNaruMizenMatcher.matches(token),
        CustomMatcher::IiForm => IiFormMatcher.matches(token),
        CustomMatcher::IkenaiForm => IkenaiFormMatcher.matches(token),
        CustomMatcher::MashouForm => MashouFormMatcher.matches(token),
        CustomMatcher::MasenForm => MasenFormMatcher.matches(token),
        CustomMatcher::MashiForm => MashiFormMatcher.matches(token),
        CustomMatcher::DeshiForm => DeshiFormMatcher.matches(token),
        CustomMatcher::ImperativeForm => ImperativeFormMatcher.matches(token),
        CustomMatcher::TariParticle => TariParticleMatcher.matches(token),
        CustomMatcher::DeshouForm => DeshouFormMatcher.matches(token),
        CustomMatcher::DeshouPreceding => DeshouPrecedingMatcher.matches(token),
        CustomMatcher::NDesuForm => NDesuFormMatcher.matches(token),
        CustomMatcher::YokattaForm => YokattaFormMatcher.matches(token),
        CustomMatcher::TagaruForm => TagaruFormMatcher.matches(token),
        CustomMatcher::ToIiForm => ToIiFormMatcher.matches(token),
        CustomMatcher::ShiParticle => ShiParticleMatcher.matches(token),
        CustomMatcher::SugiruStem => SugiruStemMatcher.matches(token),
        CustomMatcher::SouHearsayStem => SouHearsayStemMatcher.matches(token),
        CustomMatcher::MaiForm => MaiFormMatcher.matches(token),
        CustomMatcher::PpoiForm => PpoiFormMatcher.matches(token),
        CustomMatcher::TekiSuffix => TekiSuffixMatcher.matches(token),
        CustomMatcher::TateSuffix => TateSuffixMatcher.matches(token),
        CustomMatcher::GuraiForm => GuraiFormMatcher.matches(token),
        CustomMatcher::OiteForm => OiteFormMatcher.matches(token),
        CustomMatcher::NiKansuruForm => NiKansuruFormMatcher.matches(token),
        CustomMatcher::HajimeteAdverb => HajimeteAdverbMatcher.matches(token),
        CustomMatcher::OyobiConjunction => OyobiConjunctionMatcher.matches(token),
        CustomMatcher::Noun => NounMatcher.matches(token),
        CustomMatcher::DependentNounMono => DependentNounMonoMatcher.matches(token),
        CustomMatcher::VerbWithBaseSuffix(suffix) => {
            token.pos.first().is_some_and(|pos| pos == "動詞")
                && token.base_form.ends_with(suffix)
                && token.base_form != *suffix
        }
        CustomMatcher::NounWithBaseSuffix(suffix) => {
            token.pos.first().is_some_and(|pos| pos == "名詞")
                && token.base_form.ends_with(suffix)
                && token.base_form != *suffix
        }
        CustomMatcher::GaPotentialVerb => GaPotentialVerbMatcher.matches(token),
        CustomMatcher::Particle => ParticleMatcher.matches(token),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_interface() {
        // Verify trait can be used as trait object
        let _matcher: &dyn TokenMatcherLogic = &TaiFormMatcher;
    }
}
