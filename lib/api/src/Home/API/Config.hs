--------------------------------------------------------------------------------

-- | Defines `ApiConfig`, which represents the overall configuration for the
-- API service.
module Home.API.Config (
    ApiConfig,
    apiPort,
    apiPostgres,
    readApiConfig
) where

--------------------------------------------------------------------------------

import Data.Yaml

import Home.Config.SecretSource
import Home.Db.Config

--------------------------------------------------------------------------------

-- | Represents an overall configuration for the API service.
-- The type parameter @s@ is used to abstract over the representation of
-- secrets. This is intended to be `SecretSource` at the time the configuration
-- is read from disk, but may later be resolved to the actual secret values
-- in a representation such as `Text`.
data ApiConfig s = MkApiConfig {
    -- | The port on which we should listen for requests.
    apiPort :: Maybe Int,
    -- | The database configuration.
    apiPostgres :: DbConfig s
} deriving (Eq, Show, Functor, Traversable, Foldable)

instance FromJSON (ApiConfig SecretSource) where
    parseJSON :: Value -> Parser (ApiConfig SecretSource)
    parseJSON = withObject "ApiConfig" $ \v ->
        MkApiConfig <$> v .:? "port"
                    <*> v .: "postgres"

instance ToJSON (ApiConfig SecretSource) where
    toJSON :: ApiConfig SecretSource -> Value
    toJSON MkApiConfig{..} =
        object [ "port" .= apiPort
               , "postgres" .= apiPostgres
               ]

-- | `readApiConfigJson` is `decodeFileEither` specialised to `ApiConfig`.
readApiConfig
    :: FilePath
    -> IO (Either ParseException (ApiConfig SecretSource))
readApiConfig = decodeFileEither

--------------------------------------------------------------------------------
