{-# LANGUAGE UndecidableInstances #-}

module Home.API.Response (
    HasDbKey(..),
    Keyed,
    getKey,
    getModel,
    mkKeyed
) where

--------------------------------------------------------------------------------

import Data.Kind ( type Type )

import Home.JSON

--------------------------------------------------------------------------------

-- | A class for API models which correspond to database entities that can be
-- identified by some key.
class HasDbKey model where
    type family DbKey model :: Type

-- | A `Keyed` @model@ is an API model that has an associated database key.
data Keyed model = MkKeyed {
    -- | The key associated with the API model.
    getKey :: DbKey model,
    -- | The API model itself.
    getModel :: model
}

instance (ToJSON model, ToJSON (DbKey model)) => ToJSON (Keyed model) where
    toJSON :: Keyed model -> Value
    toJSON (MkKeyed key val) =
        object [ "id" .= key
               , "data" .= val
               ]

instance (FromJSON model, FromJSON (DbKey model)) => FromJSON (Keyed model) where
    parseJSON = withObject "Keyed" $ \obj ->
        MkKeyed <$> obj .: "id" <*> obj .: "data"

mkKeyed :: DbKey model -> model -> Keyed model
mkKeyed = MkKeyed

--------------------------------------------------------------------------------
