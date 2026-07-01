use iac_macro::iac;

fn main() {
  iac! {
      bucket uniquename
  }

  iac! {
      // won't accept '-', not valid for identifiers
      lambda a_name
  }

  iac! {
      lambda my_name mem 1024 time 15
  }

  iac! {
      bucket uniquename lambda anothername
  }

  iac! {
      lambda name bucket anothername
  }

  iac! {
      bucket uniquename => lambda anothername
  }

  iac! {
      bucket uniquename => lambda anothername mem 1024 time 15
  }
}
