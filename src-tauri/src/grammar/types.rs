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
    Must, // nakereba naranai / nakute wa ikenai
}

// Implementation removed - genki_chapter method was unused

// JLPT grammar patterns removed - only JLPT vocabulary stats remain in episode_jlpt_stats table
// PartOfSpeech enum removed - POS data is stored as JSON Vec<String> from Kagome