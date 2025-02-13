use convert_case::Casing;

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
)]
pub enum PostalCommunity {
    /// The City of Grants Pass, an incorporated municipality and unincorporated community.
    #[default]
    GrantsPass,
    /// The City of Medford, an incorporated municipality.
    Medford,
    /// The City of Merlin, an unincorporated community.
    Merlin,
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
        match input.to_lowercase().as_str() {
            "grants pass" => Some(Self::GrantsPass),
            "gp" => Some(Self::GrantsPass),
            "medford" => Some(Self::Medford),
            "merlin" => Some(Self::Merlin),
            _ => None,
        }
    }
}
