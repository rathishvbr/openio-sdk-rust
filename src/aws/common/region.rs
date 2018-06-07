// Copyright 2017 LambdaStack All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

// Portions borrowed from the rusoto project. See README.md
//

//! AWS Regions and helper functions.
//!
//! Mostly used for translating the Region enum to a string AWS accepts.
//!
//! For example: `UsEast1` to "us-east-1"

use std::error::Error;
use std::str::FromStr;
use std::fmt::{Display, Error as FmtError, Formatter};

/// An AWS region.
/// CnNorth1 is currently untested due to Rusoto maintainers not having access to AWS China.
#[derive(Debug, Clone, Copy, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Region {
    ApNortheast1,
    ApNortheast2,
    ApSouth1,
    ApSoutheast1,
    ApSoutheast2,
    EuCentral1,
    EuWest1,
    SaEast1,
    UsEast1,
    UsWest1,
    UsWest2,
    CnNorth1,
}

/// An error produced when attempting to convert a `str` into a `Region` fails.
#[derive(Debug, PartialEq, Default, RustcDecodable, RustcEncodable)]
pub struct ParseRegionError {
    message: String,
}

impl Display for Region {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        let region_str = match *self {
            Region::ApNortheast1 => "ap-northeast-1",
            Region::ApNortheast2 => "ap-northeast-2",
            Region::ApSouth1 => "ap-south-1",
            Region::ApSoutheast1 => "ap-southeast-1",
            Region::ApSoutheast2 => "ap-southeast-2",
            Region::EuCentral1 => "eu-central-1",
            Region::EuWest1 => "eu-west-1",
            Region::SaEast1 => "sa-east-1",
            Region::UsEast1 => "us-east-1",
            Region::UsWest1 => "us-west-1",
            Region::UsWest2 => "us-west-2",
            Region::CnNorth1 => "cn-north-1",
        };

        write!(f, "{}", region_str)
    }
}

impl FromStr for Region {
    type Err = ParseRegionError;

    fn from_str(s: &str) -> Result<Region, ParseRegionError> {
        match s {
            "ap-northeast-1" => Ok(Region::ApNortheast1),
            "ap-northeast-2" => Ok(Region::ApNortheast2),
            "ap-south-1" => Ok(Region::ApSouth1),
            "ap-southeast-1" => Ok(Region::ApSoutheast1),
            "ap-southeast-2" => Ok(Region::ApSoutheast2),
            "eu-central-1" => Ok(Region::EuCentral1),
            "eu-west-1" => Ok(Region::EuWest1),
            "sa-east-1" => Ok(Region::SaEast1),
            "us-east-1" => Ok(Region::UsEast1),
            "us-west-1" => Ok(Region::UsWest1),
            "us-west-2" => Ok(Region::UsWest2),
            "cn-north-1" => Ok(Region::CnNorth1),
          //  s => Err(ParseRegionError::new(s)),
          s => Ok(Region::EuWest1),
        }
    }
}

impl ParseRegionError {
    pub fn new(input: &str) -> Self {
        ParseRegionError { message: format!("Not a valid AWS region: {}", input) }
    }
}

impl Error for ParseRegionError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl Display for ParseRegionError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!("foo"
                       .parse::<Region>()
                       .err()
                       .expect("Parsing foo as a Region was not an error")
                       .to_string(),
                   "Not a valid AWS region: foo".to_owned());
        assert_eq!("ap-northeast-1".parse(), Ok(Region::ApNortheast1));
        assert_eq!("ap-northeast-2".parse(), Ok(Region::ApNortheast2));
        assert_eq!("ap-south-1".parse(), Ok(Region::ApSouth1));
        assert_eq!("ap-southeast-1".parse(), Ok(Region::ApSoutheast1));
        assert_eq!("ap-southeast-2".parse(), Ok(Region::ApSoutheast2));
        assert_eq!("eu-central-1".parse(), Ok(Region::EuCentral1));
        assert_eq!("eu-west-1".parse(), Ok(Region::EuWest1));
        assert_eq!("sa-east-1".parse(), Ok(Region::SaEast1));
        assert_eq!("us-east-1".parse(), Ok(Region::UsEast1));
        assert_eq!("us-west-1".parse(), Ok(Region::UsWest1));
        assert_eq!("us-west-2".parse(), Ok(Region::UsWest2));
        assert_eq!("cn-north-1".parse(), Ok(Region::CnNorth1));
    }

    #[test]
    fn region_display() {
        assert_eq!(Region::ApNortheast1.to_string(),
                   "ap-northeast-1".to_owned());
        assert_eq!(Region::ApNortheast2.to_string(),
                   "ap-northeast-2".to_owned());
        assert_eq!(Region::ApSouth1.to_string(), "ap-south-1".to_owned());
        assert_eq!(Region::ApSoutheast1.to_string(),
                   "ap-southeast-1".to_owned());
        assert_eq!(Region::ApSoutheast2.to_string(),
                   "ap-southeast-2".to_owned());
        assert_eq!(Region::EuCentral1.to_string(), "eu-central-1".to_owned());
        assert_eq!(Region::EuWest1.to_string(), "eu-west-1".to_owned());
        assert_eq!(Region::SaEast1.to_string(), "sa-east-1".to_owned());
        assert_eq!(Region::UsEast1.to_string(), "us-east-1".to_owned());
        assert_eq!(Region::UsWest1.to_string(), "us-west-1".to_owned());
        assert_eq!(Region::UsWest2.to_string(), "us-west-2".to_owned());
        assert_eq!(Region::CnNorth1.to_string(), "cn-north-1".to_owned());
    }
}
