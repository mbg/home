--------------------------------------------------------------------------------

-- | Defines `ApiConfig`, which represents the overall configuration for the
-- API service.
module Home.API.Config (
    ApiConfig,
    apiPort,
    readApiConfig
) where

--------------------------------------------------------------------------------

import Data.Yaml

--------------------------------------------------------------------------------

data ApiConfig = MkApiConfig {
    -- | The port on which we should listen for requests.
    apiPort :: Maybe Int
} deriving (Eq, Show)

instance FromJSON ApiConfig where
    parseJSON :: Value -> Parser ApiConfig
    parseJSON = withObject "ApiConfig" $ \v ->
        MkApiConfig <$> v .:? "port"

instance ToJSON ApiConfig where
    toJSON :: ApiConfig -> Value
    toJSON MkApiConfig{..} =
        object [ "port" .= apiPort
               ]

-- | `readApiConfigJson` is `decodeFileEither` specialised to `ApiConfig`.
readApiConfig :: FilePath -> IO (Either ParseException ApiConfig)
readApiConfig = decodeFileEither

--------------------------------------------------------------------------------
