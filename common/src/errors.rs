use atspi_common::{AtspiError};
use serde_plain::Error as SerdePlainError;
use std::{error::Error, fmt};

#[non_exhaustive]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum OdiliaError {
	/// See: [`atspi_common::error::AtspiError`].
	Atspi(AtspiError),
	/// Issue converting between some other type and [`AccessiblePrimitive`].
	PrimitiveConversionError(AccessiblePrimitiveConversionError),
	/// This error occurs when you attempt to convert an [`crate::state::OdiliaState`] into a specific variant, and the variant contained within the enum is incorrect.
	InvalidStateVariant,
	/// See: [`CacheError`].
	Cache(CacheError),
	/// An error that should never happen. It's merely here just to please the compiler in rare cases.
	InfallibleConversion,
	/// A parsing error converting a string into a type (usually an integer).
	/// The error message is preserved through the `String` variant data.
	ParseError(String),
	/// See: [`ConfigError`].
	Config(ConfigError),
	/// A generic error type where the error message is preserved, but it is not enumerable.
	/// These are the kind of errors that generally should have a [bug filed](https://github.com/odilia-app/odilia/issues) for them.
	Generic(String),
}

/// Errors when loading or reading from settings.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ConfigError {
	/// [`tini`] errors are converted into a string. These are usually errors about parsing the file.
	Tini(String),
	/// The value requested could not be found.
	ValueNotFound,
	/// The path of the file to load the config from could not be found.
	PathNotFound,
}
#[cfg(feature = "zbus")]
impl From<zbus::Error> for OdiliaError {
	fn from(z_err: zbus::Error) -> Self {
		OdiliaError::Generic(z_err.to_string())
	}
}
#[cfg(feature = "zbus")]
impl From<zbus::fdo::Error> for OdiliaError {
	fn from(z_err: zbus::fdo::Error) -> Self {
		OdiliaError::Generic(z_err.to_string())
	}
}
impl From<tini::Error> for ConfigError {
	fn from(t_err: tini::Error) -> Self {
		Self::Tini(t_err.to_string())
	}
}
impl std::fmt::Display for ConfigError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Tini(t) => t.fmt(f),
			Self::ValueNotFound => f.write_str("Vlaue not found in config file."),
			Self::PathNotFound => {
				f.write_str("The path for the config file was not found.")
			}
		}
	}
}
impl std::error::Error for ConfigError {}
/// Errors when dealing with Odilia's cache.
/// The types are defined in [`crate::cache`], but the implementation is in an external crate.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum CacheError {
	/// The cache is not avaialbe.
	NotAvailable,
	/// The item requested was not found.
	NoItem,
	/// A lock (read or write) could not be aquired on the cache.
	NoLock,
}
impl std::fmt::Display for CacheError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::NotAvailable => f.write_str("The cache has been dropped from memory. This never happens under normal circumstances, and should never happen. Please send a detailed bug report if this ever happens."),
			Self::NoItem => f.write_str("No item in cache found."),
      Self::NoLock => f.write_str("It was not possible to get a lock on this item from the cache."),
		}
	}
}
impl std::error::Error for CacheError {}
impl Error for OdiliaError {}
impl From<std::num::TryFromIntError> for OdiliaError {
	fn from(fie: std::num::TryFromIntError) -> Self {
		Self::ParseError(fie.to_string())
	}
}
impl From<std::convert::Infallible> for OdiliaError {
	fn from(_infallible: std::convert::Infallible) -> Self {
		Self::InfallibleConversion
	}
}
impl From<CacheError> for OdiliaError {
	fn from(cache_error: CacheError) -> Self {
		Self::Cache(cache_error)
	}
}
impl From<zvariant::Error> for OdiliaError {
	fn from(spe: zvariant::Error) -> Self {
		Self::Atspi(spe.into())
	}
}
impl From<SerdePlainError> for OdiliaError {
	fn from(spe: SerdePlainError) -> Self {
		Self::Generic(spe.to_string())
	}
}
impl From<AtspiError> for OdiliaError {
	fn from(err: AtspiError) -> OdiliaError {
		Self::Atspi(err)
	}
}
impl fmt::Display for OdiliaError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{self:?}")
	}
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum AccessiblePrimitiveConversionError {
	ParseError,
	NoPathId,
	InvalidPath,
	NoFirstSectionOfSender,
	NoSecondSectionOfSender,
	NoSender,
	ErrSender,
}
impl From<AccessiblePrimitiveConversionError> for OdiliaError {
	fn from(apc_error: AccessiblePrimitiveConversionError) -> Self {
		Self::PrimitiveConversionError(apc_error)
	}
}
impl fmt::Display for AccessiblePrimitiveConversionError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{self:?}")
	}
}
impl std::error::Error for AccessiblePrimitiveConversionError {}
