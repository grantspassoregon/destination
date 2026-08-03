use convert_case::Casing;
use std::str::FromStr;

/// The `PostalCommunity` enum holds valid variants for the postal community field of an address.
/// The list of valid postal communities is limited to the set of communities encountered locally,
/// and we add new variants as needed.
///
/// This enum exists to facilitate parsing addresses.  Because street names and types can parse
/// ambiguously, it can be unclear during a parse whether a word should be parsed as a street name,
/// street type or postal community.  By ensuring the postal community maps to a valid value, we
/// can reduce the risk of parsing a word to the wrong address element.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
    schemars::JsonSchema,
    elicitation::Elicit,
)]
#[allow(missing_docs)]
pub enum PostalCommunity {
    /// The City of Grants Pass, an incorporated municipality and unincorporated community.
    #[default]
    GrantsPass,
    /// The City of Medford, an incorporated municipality.
    Medford,
    /// The City of Merlin, an unincorporated community.
    Merlin,
    Addison,
    Albany,
    Aloha,
    Alpharetta,
    Applegate,
    Arlington,
    Armonk,
    Ashland,
    Atlanta,
    Azalea,
    Bakersfield,
    BattleGround,
    Beaverton,
    Bellevue,
    Bend,
    Bentonville,
    Berkeley,
    Birmingham,
    BocaRaton,
    Boise,
    Bothell,
    Brea,
    Brentwood,
    Brookings,
    Broomfield,
    Brownsville,
    BrushPrarie,
    Buda,
    Calabasas,
    Camas,
    Canby,
    Canonsburg,
    Canyonville,
    Carlsbad,
    CaveJunction,
    CentralPoint,
    Chesapeake,
    Chubbuck,
    Cincinnati,
    Clackamas,
    Clayton,
    Clearwater,
    Cleveland,
    Columbus,
    Coquille,
    Corvallis,
    CostaMesa,
    Cranberry,
    Dallas,
    DaytonaBeach,
    Deerfield,
    Denton,
    Denver,
    Dixon,
    Dorris,
    Dublin,
    Duluth,
    Durham,
    EaglePoint,
    EdenPrairie,
    Enterprise,
    Estacada,
    Eugene,
    Fairfield,
    Fife,
    Fishers,
    Florence,
    FortWorth,
    Framingham,
    Fresno,
    Glendale,
    GlendaleHeights,
    GoldHill,
    Goodlettsville,
    Grapevine,
    GreatFalls,
    Hillsboro,
    Houston,
    Hubbard,
    Independence,
    Indianapolis,
    Industry,
    Irvine,
    Irving,
    Jacksonville,
    Joshua,
    JunctionCity,
    Keizer,
    Kent,
    Kerby,
    KingOfPrussia,
    KlamathFalls,
    Lafayette,
    LakeOswego,
    LasVegas,
    Lehi,
    Lenexa,
    LibertyLake,
    Logan,
    LongBeach,
    Longview,
    LosAngeles,
    Louisville,
    Lynnwood,
    Maitland,
    Mason,
    Maxwell,
    Mayville,
    Memphis,
    Meridian,
    Milwaukie,
    Minneapolis,
    Monsey,
    Murphy,
    MyrtlePoint,
    Nashville,
    NorthCanton,
    Norwell,
    Novato,
    Ogden,
    Olympia,
    Orange,
    Pacific,
    PaloCedro,
    ParadiseValley,
    Pasadena,
    Pendleton,
    Petaluma,
    Pheonix,
    Piru,
    Pittsburgh,
    Portland,
    Princeton,
    Providence,
    Provo,
    RedBluff,
    Redding,
    Redmond,
    Reston,
    Riddle,
    RogueRiver,
    Roseburg,
    Rosemead,
    Roseville,
    Sacramento,
    SaintLouis,
    Salem,
    SaltLakeCity,
    SanAntonio,
    SanDiego,
    SanDimas,
    SanFrancisco,
    SanRamon,
    SandySprings,
    Seattle,
    Selma,
    ShadyCove,
    Sherwood,
    Silverton,
    Spokane,
    SpokaneValley,
    Springfield,
    SunnyValley,
    Talent,
    Tampa,
    Temecula,
    Tempe,
    Temple,
    Toledo,
    Trail,
    Tualatin,
    Turner,
    Ukiah,
    Umatilla,
    Vancouver,
    Ventura,
    VirginiaBeach,
    Visalia,
    Westlake,
    WhiteCity,
    WhitePlains,
    Wilderville,
    Williams,
    Wilmington,
    Winchester,
    WolfCreek,
    Woonsocket,
    Yakima,
    Yreka,
    YubaCity,
}

impl PostalCommunity {
    /// The `label` method returns the name of the community in all caps with spaces, for printing
    /// labels.
    ///
    /// ```
    /// use destination::PostalCommunity;
    ///
    /// let city = PostalCommunity::GrantsPass;
    ///
    /// assert_eq!(&city.label(), "GRANTS PASS");
    /// ```
    #[tracing::instrument]
    pub fn label(&self) -> String {
        let title = self.to_string().to_case(convert_case::Case::Title);
        title.to_uppercase()
    }

    /// The `match_mixed` method attempts to match the string `input` against a variant of
    /// `PostalCommunity`.  Used to parse the postal community from an address blob.
    ///
    /// ```
    /// use destination::PostalCommunity;
    ///
    /// let a = PostalCommunity::match_mixed("Grants Pass").unwrap();
    /// let b = PostalCommunity::match_mixed("GP").unwrap();
    ///
    /// assert_eq!(a, b);
    /// ```
    #[tracing::instrument]
    pub fn match_mixed(input: &str) -> Option<Self> {
        let mut candidate = input.to_string();
        candidate.retain(|c| !c.is_whitespace());
        if let Ok(value) = PostalCommunity::from_str(&candidate) {
            return Some(value);
        }

        match input.to_lowercase().as_str() {
            "gp" => Some(Self::GrantsPass),
            _ => None,
        }
    }
}
