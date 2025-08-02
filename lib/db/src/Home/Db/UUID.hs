--------------------------------------------------------------------------------

{-# LANGUAGE DerivingStrategies #-}
{-# LANGUAGE GeneralizedNewtypeDeriving #-}

-- | This module implements the `Id` type, which we use for unique identifiers
-- throughout the database schema. This is a wrapper around `UUID.UUID` so that
-- we can implement additional type class instances without needing to worry
-- about orphan instances.
module Home.Db.UUID (
    Id(..),
    nilId,
    nextRandom,
    toText,
    fromASCIIBytes,
    uuidDef
) where

--------------------------------------------------------------------------------

import Control.Monad.IO.Class

import Data.Aeson
import Data.ByteString ( ByteString )
import Data.ByteString.Char8 ( pack, unpack )
import Data.Maybe
import Data.Proxy ( Proxy )
import Data.String
import Data.Text ( Text )
import Data.UUID.Types qualified as UUID
import Data.UUID.V4 qualified as UUID

import Database.Persist
import Database.Persist.ImplicitIdDef
import Database.Persist.Sql

import Web.HttpApiData
import Web.PathPieces

--------------------------------------------------------------------------------

-- | We use UUIDs as identifiers throughout the database schema, represented by
-- this type. This is a wrapper around the `UUID` type to avoid orphan instances.
newtype Id = MkId { getId :: UUID.UUID }
    deriving stock ( Eq, Ord, Read, Show )
    deriving newtype ( FromJSON, ToJSON
                     , FromHttpApiData, ToHttpApiData
                     )

-- | `nilId` represents an all-zero `Id`.
nilId :: Id
nilId = MkId { getId = UUID.nil }

-- | `nextRandom` generates a random `Id`.
nextRandom :: MonadIO m => m Id
nextRandom = MkId <$> liftIO UUID.nextRandom

-- | `toText` @id@ converts @id@ to a `Text` value.
toText :: Id -> Text
toText = UUID.toText . getId

-- | `fromASCIIBytes` @bytestring@ tries to parse @bytestring@ as an `Id`.
fromASCIIBytes :: ByteString -> Maybe Id
fromASCIIBytes = fmap MkId . UUID.fromASCIIBytes

instance PathPiece Id where
    fromPathPiece :: Text -> Maybe Id
    fromPathPiece = fmap MkId . UUID.fromText

    toPathPiece :: Id -> Text
    toPathPiece = UUID.toText . getId

instance PersistField Id where
    toPersistValue :: Id -> PersistValue
    toPersistValue = PersistLiteral_ Escaped . pack . UUID.toString . getId

    fromPersistValue :: PersistValue -> Either Text Id
    fromPersistValue (PersistLiteral_ Escaped bs) =
        case UUID.fromString $ unpack bs of
            Just uuid -> Right $ MkId { getId = uuid }
            Nothing -> Left "Invalid UUID"
    fromPersistValue _ = Left "Invalid UUID storage type"

instance PersistFieldSql Id where
    sqlType :: Proxy Id -> SqlType
    sqlType _ = SqlOther "uuid"

-- | For convenience only: The `fromString` method of the `IsString` class
-- is unsafe.
instance IsString Id where
    fromString :: String -> Id
    fromString =
        MkId .
        fromMaybe (error "IsString(Id)/fromString: not a valid UUID") .
        UUID.fromString

-- | An `ImplicitIdDef` for `Id` which is used to tell persistent what value
-- to use as a default for columns of the `Id` type.
uuidDef :: ImplicitIdDef
uuidDef = mkImplicitIdDef @Id "uuid_generate_v4()"

--------------------------------------------------------------------------------
