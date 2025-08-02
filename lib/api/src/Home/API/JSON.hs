
module Home.API.JSON (
    JSONOptions,
    -- * Re-exports
    Generic,
    module Data.Aeson,
    CustomJSON(..)
) where

--------------------------------------------------------------------------------

import Data.Aeson

import Deriving.Aeson

import GHC.TypeLits

--------------------------------------------------------------------------------

-- | Our custom JSON encoding and decoding options, for use with `CustomJSON`.
type JSONOptions (prefix :: Symbol) =
  '[OmitNothingFields, FieldLabelModifier '[StripPrefix prefix, CamelToSnake]]

--------------------------------------------------------------------------------
