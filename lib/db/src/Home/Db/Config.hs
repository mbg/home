--------------------------------------------------------------------------------

-- | Exports types to represent database configurations as well as
-- associated functions.
module Home.Db.Config (
    DbConfig,
    toConnStr
) where

--------------------------------------------------------------------------------

import Data.Aeson
import Data.Aeson.Types (Parser)
import Data.Text ( Text )
import Data.Text qualified as T
import Data.Text.Encoding ( encodeUtf8 )
import Data.ByteString
import Data.Maybe

import Home.Config.SecretSource

--------------------------------------------------------------------------------

-- | Represents database configurations.
data DbConfig s = MkDbConfig {
    -- | (Optional) The hostname of the server.
    -- We use @localhost@ by default if this is not set.
    dbConfigHost :: Maybe Text,
    -- | (Optional) The port on which the server listens for connections.
    -- We default to the standard PostgreSQL port of @5432@ if this is not set.
    dbConfigPort :: Maybe Int,
    -- | (Optional) The username needed for authentication to the server.
    -- Defaults to @postgres@ if not set.
    dbConfigUser :: Maybe Text,
    -- | (Optional) The password needed for authentication to the server.
    -- Defaults to the empty string if not set.
    dbConfigPassword :: Maybe s,
    -- | (Optional) The name of the database.
    -- Defaults to @postgres@ if not set.
    dbConfigDb :: Maybe Text
} deriving (Eq, Show, Functor, Traversable, Foldable)

instance FromJSON (DbConfig SecretSource) where
    parseJSON :: Value -> Parser (DbConfig SecretSource)
    parseJSON = withObject "DbConfig" $ \v ->
        MkDbConfig <$> v .:? "host"
                   <*> v .:? "port"
                   <*> v .:? "user"
                   <*> v .:? "password"
                   <*> v .:? "db"

instance ToJSON (DbConfig SecretSource) where
    toJSON :: DbConfig SecretSource -> Value
    toJSON MkDbConfig{..} =
        object [ "host" .= dbConfigHost
               , "port" .= dbConfigPort
               , "user" .= dbConfigUser
               , "password" .= dbConfigPassword
               , "db" .= dbConfigDb
               ]

-- | `toConnStr` @config@ constructs a connection string from @config@ that
-- is suitable for PostgreSQL servers.
toConnStr :: DbConfig Text -> ByteString
toConnStr MkDbConfig{..} = encodeUtf8 $ T.concat
    [ "host=", fromMaybe "localhost" dbConfigHost
    , " port=", T.pack (show $ fromMaybe 5432 dbConfigPort)
    , " user=", fromMaybe "postgres" dbConfigUser
    , " password=", fromMaybe "" dbConfigPassword
    , " dbname=", fromMaybe "postgres" dbConfigDb
    ]

--------------------------------------------------------------------------------
