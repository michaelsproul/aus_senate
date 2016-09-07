# Ballot Parsing

Several checks are possible:

* Only above, only below, or both (prioritising one or the other).
* Minimum and maximum numbers of preferences above and below the line.
    2016 strict: minimum 6 above, minimum 12 below.
    2016 lax: minimum 1 above, minimum 1 below.
    2013 strict: minimum and maximum 1 above, minimum all below.

    Defaults:
        min above: 1
        max above: all
        min below: 1
        max below: all

Algorithm:

* Parse both above and below the line votes, according to the count constraints.
  Return a Result<Vec<CandidateId>, BallotParseError>
* Apply the choice constraint to choose between the above and below the line votes.
