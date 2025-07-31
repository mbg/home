
-- | Implements the `SecretSource` type, which is used to represent references
-- to secrets in configuration files.
module Home.Config.SecretSource (
    SecretSource(..),
    resolveSecret
) where

--------------------------------------------------------------------------------

import Data.Aeson
import Data.Aeson.Types
import Data.Aeson.KeyMap as KM
import Data.Text

import System.Environment
import System.Directory
import System.IO

--------------------------------------------------------------------------------

-- | Represents a reference to a secret in a configuration file.
data SecretSource
    -- | The value is stored in an environment variable with the given name.
    = EnvSecret Text
    -- | The value is stored in a file at the given path.
    | FileSecret FilePath
    deriving (Eq, Show)

instance ToJSON SecretSource where
    toJSON :: SecretSource -> Value
    toJSON (EnvSecret varName) = object [ "secretVar" .= varName ]
    toJSON (FileSecret path) = object [ "secretFile" .= path ]

instance FromJSON SecretSource where
    parseJSON :: Value -> Parser SecretSource
    parseJSON = withObject "SecretSource" $
        \obj -> case KM.lookup "secretVar" obj of
            Just var -> EnvSecret <$> parseJSON var
            Nothing -> case KM.lookup "secretFile" obj of
                Just fp -> FileSecret <$> parseJSON fp
                Nothing -> fail "Invalid specification for a secret."


-- | `resolveSecret` @secretSource@ is a computation which attempts to resolve
-- @secretSource@ to its corresponding value.
resolveSecret :: SecretSource -> IO (Maybe Text)
resolveSecret (EnvSecret var) =
    fmap pack <$> lookupEnv (unpack var)
resolveSecret (FileSecret path) = do
    exists <- doesFileExist path
    if exists then withFile path ReadMode $ \fp ->
        Just . pack <$> hGetContents fp
    else pure Nothing

--------------------------------------------------------------------------------
