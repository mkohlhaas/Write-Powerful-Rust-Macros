pub trait MarkerTraitForBuilder {}
pub struct roots_ofOfGleipnirBuilder {}
impl MarkerTraitForBuilder for roots_ofOfGleipnirBuilder {}
pub struct breath_of_a_fishOfGleipnirBuilder {}
impl MarkerTraitForBuilder for breath_of_a_fishOfGleipnirBuilder {}
pub struct anything_elseOfGleipnirBuilder {}
impl MarkerTraitForBuilder for anything_elseOfGleipnirBuilder {}
pub struct FinalBuilder {}
impl MarkerTraitForBuilder for FinalBuilder {}
pub struct GleipnirBuilder<T: MarkerTraitForBuilder> {
  marker: std::marker::PhantomData<T>,
  roots_of: Option<String>,
  breath_of_a_fish: Option<u8>,
  anything_else: Option<bool>,
}
impl Gleipnir {
  pub fn builder() -> GleipnirBuilder<roots_ofOfGleipnirBuilder> {
    GleipnirBuilder {
      marker: Default::default(),
      roots_of: None,
      breath_of_a_fish: None,
      anything_else: None,
    }
  }
}
impl GleipnirBuilder<anything_elseOfGleipnirBuilder> {
  pub fn anything_else(mut self, input: bool) -> GleipnirBuilder<FinalBuilder> {
    self.anything_else = Some(input);
    GleipnirBuilder {
      marker: Default::default(),
      roots_of: self.roots_of,
      breath_of_a_fish: self.breath_of_a_fish,
      anything_else: self.anything_else,
    }
  }
}
impl GleipnirBuilder<breath_of_a_fishOfGleipnirBuilder> {
  pub fn breath_of_a_fish(mut self, input: u8) -> GleipnirBuilder<anything_elseOfGleipnirBuilder> {
    self.breath_of_a_fish = Some(input);
    GleipnirBuilder {
      marker: Default::default(),
      roots_of: self.roots_of,
      breath_of_a_fish: self.breath_of_a_fish,
      anything_else: self.anything_else,
    }
  }
}
impl GleipnirBuilder<roots_ofOfGleipnirBuilder> {
  pub fn roots_of(mut self, input: String) -> GleipnirBuilder<breath_of_a_fishOfGleipnirBuilder> {
    self.roots_of = Some(input);
    GleipnirBuilder {
      marker: Default::default(),
      roots_of: self.roots_of,
      breath_of_a_fish: self.breath_of_a_fish,
      anything_else: self.anything_else,
    }
  }
}
impl GleipnirBuilder<FinalBuilder> {
  pub fn build(self) -> Gleipnir {
    Gleipnir {
      roots_of: self.roots_of.expect(concat!("Field not set: ", "roots_of")),
      breath_of_a_fish: self
        .breath_of_a_fish
        .expect(concat!("Field not set: ", "breath_of_a_fish")),
      anything_else: self
        .anything_else
        .expect(concat!("Field not set: ", "anything_else")),
    }
  }
}
