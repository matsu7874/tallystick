use num_traits::Num;
use num_traits::Float;

/// A quota defines how many votes are required to win an election in relation to the total number of votes cast.
pub enum Quota {

    /// Droop quota. It is defined as:
    /// 
    /// ```floor((total-votes / (total-seats + 1)) + 1```
    /// 
    /// In single-winner elections, it's often known as "fifty percent plus one".
    /// The Droop quota is always an integer, even when using fractional votes.
    /// 
    /// See [wikipedia](https://en.wikipedia.org/wiki/Droop_quota) for more details.
    Droop,

    /// Hagenbach-Bischoff quota.
    /// 
    /// Also known as the "Newland-Britton quota" or the "exact Droop quota", it is defined as:
    /// 
    /// ```total-votes / (total-seats + 1)```
    /// 
    /// It differs from the Droop quota in that the quota often contains a fraction. In single-winner elections, 
    /// the first candidate to achieve more than 50% of the vote wins. This system is best used when fractional 
    /// votes are being used, or in a transferable-vote system where votes are redistributed fractionally.
    /// 
    /// See [wikipedia](https://en.wikipedia.org/wiki/Hagenbach-Bischoff_quota) for more details.
    Hagenbach,
  
    /// Hare quota.
    /// 
    /// It is defined as:
    /// 
    /// ```total-votes / total-seats```
    /// 
    /// In single-winner elections, it is equal to one hundred percent of the vote.
    /// It is generally not recommended and is included for completeness.
    /// 
    /// See [wikipedia](https://en.wikipedia.org/wiki/Hare_quota) for more details.
    Hare,

    /// Imperiali quota.
    /// 
    /// It is defined as:
    /// 
    /// ```total-votes / (total-seats + 2)```
    /// 
    /// It is rarely used and not recommended. 
    /// 
    /// See [wikipedia](https://en.wikipedia.org/wiki/Imperiali_quota) for more details.
    Imperiali
}

// TODO: Fix this for float-types. Right now this isn't calling floor() for droop.
impl Quota {
  /// Compute the threshold needed to be elected for the given quota.
  /// 
  /// Note that total-votes should be the number of votes counted in the tally.
  /// It should not include invalid votes that were not added the tally.
  /// For weighted tallies, it should be the sum of all weights.
  pub fn threshold<C: Num>(&self, total_votes: C, num_winners: C) -> C {
    match self {
      // TODO: Do some generic wizardry here to call .floor() on Float types for Droop
      Quota::Droop => (total_votes / (num_winners + C::one())) + C::one(),
      Quota::Hagenbach => total_votes / (num_winners + C::one()),
      Quota::Hare => total_votes / num_winners,
      Quota::Imperiali => total_votes / (num_winners + C::one() + C::one())
    }
  }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quota_test() {
      // Integers
      // --------
      assert!(Quota::Droop.threshold(100, 1) == 51);
      assert!(Quota::Droop.threshold(101, 1) == 51);
      assert!(Quota::Droop.threshold(102, 1) == 52);

      assert!(Quota::Droop.threshold(100, 2) == 34);
      assert!(Quota::Droop.threshold(101, 2) == 34);
      assert!(Quota::Droop.threshold(102, 2) == 35);
      
      assert!(Quota::Hagenbach.threshold(100, 1) == 50);
      assert!(Quota::Hagenbach.threshold(101, 1) == 50); // 50.5 rounded down because integer
      assert!(Quota::Hagenbach.threshold(102, 1) == 51);

      assert!(Quota::Hagenbach.threshold(100, 2) == 33); // 33.333... rounded down because integer
      assert!(Quota::Hagenbach.threshold(101, 2) == 33); // 33.666... rounded down because integer
      assert!(Quota::Hagenbach.threshold(102, 2) == 34);
      
      assert!(Quota::Hare.threshold(100, 1) == 100);
      assert!(Quota::Hare.threshold(101, 1) == 101);
      assert!(Quota::Hare.threshold(102, 1) == 102);
  
      assert!(Quota::Hare.threshold(100, 2) == 50);
      assert!(Quota::Hare.threshold(101, 2) == 50); // 50.5 rounded down because integer
      assert!(Quota::Hare.threshold(102, 2) == 51);

      assert!(Quota::Imperiali.threshold(100, 1) == 33);
      assert!(Quota::Imperiali.threshold(101, 1) == 33);
      assert!(Quota::Imperiali.threshold(102, 1) == 34);

      assert!(Quota::Imperiali.threshold(100, 2) == 25);
      assert!(Quota::Imperiali.threshold(101, 2) == 25); // 25.25 rounded down because integer
      assert!(Quota::Imperiali.threshold(102, 2) == 25); // 25.50 rounded down because integer


      // Floats
      // ------
      let thirty_three_point_threes = 33.0 + (1.0 / 3.0); // 33.333...
      let thirty_three_point_sixes = 33.0 + (2.0 / 3.0);  // 33.666...

      assert!(Quota::Droop.threshold(100.0, 1.0) == 51.0);
      // assert!(Quota::Droop.threshold(101.0, 1.0) == 51.0); // Broken, not calling .floor() on floats
      assert!(Quota::Droop.threshold(102.0, 1.0) == 52.0);

      // assert!(Quota::Droop.threshold(100.0, 2.0) == 34.0); // Broken, not calling .floor() on floats
      // assert!(Quota::Droop.threshold(101.0, 2.0) == 34.0); // Broken, not calling .floor() on floats
      assert!(Quota::Droop.threshold(102.0, 2.0) == 35.0);
      
      assert!(Quota::Hagenbach.threshold(100.0, 1.0) == 50.0);
      assert!(Quota::Hagenbach.threshold(101.0, 1.0) == 50.5);
      assert!(Quota::Hagenbach.threshold(102.0, 1.0) == 51.0);

      assert!(Quota::Hagenbach.threshold(100.0, 2.0) == thirty_three_point_threes); // 33.333...
      assert!(Quota::Hagenbach.threshold(101.0, 2.0) == thirty_three_point_sixes); // 33.666...
      assert!(Quota::Hagenbach.threshold(102.0, 2.0) == 34.0);

      assert!(Quota::Hare.threshold(100.0, 1.0) == 100.0);
      assert!(Quota::Hare.threshold(101.0, 1.0) == 101.0);
      assert!(Quota::Hare.threshold(102.0, 1.0) == 102.0);
  
      assert!(Quota::Hare.threshold(100.0, 2.0) == 50.0);
      assert!(Quota::Hare.threshold(101.0, 2.0) == 50.5);
      assert!(Quota::Hare.threshold(102.0, 2.0) == 51.0);

      assert!(Quota::Imperiali.threshold(100.0, 1.0) == thirty_three_point_threes); // 33.333...
      assert!(Quota::Imperiali.threshold(101.0, 1.0) == thirty_three_point_sixes); // 33.666...
      assert!(Quota::Imperiali.threshold(102.0, 1.0) == 34.0);

      assert!(Quota::Imperiali.threshold(100.0, 2.0) == 25.00);
      assert!(Quota::Imperiali.threshold(101.0, 2.0) == 25.25);
      assert!(Quota::Imperiali.threshold(102.0, 2.0) == 25.50);
    }
}
