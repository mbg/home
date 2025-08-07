--------------------------------------------------------------------------------

{-# LANGUAGE ScopedTypeVariables #-}

-- | Persistent does not support creating arbitrary indices out-of-the-box.
-- This module exports the `createIndex` function, which can be used as part
-- of a persistent `Migration` to create an index if it does not already
-- exist. An index contains one or more `IndexField` values, which can be
-- constructed using `indexField`.
module Home.Db.Index (
    SortOrder(..),
    asc,
    desc,
    NullsOrder(..),
    nullsFirst,
    nullsLast,
    IndexField,
    indexField,
    createIndex
) where

--------------------------------------------------------------------------------

import Data.Proxy
import Data.Text qualified as T

import Database.Persist.Class
import Database.Persist.Sql

--------------------------------------------------------------------------------

-- | Enumerates sorting orders.
data SortOrder
    -- | Ascending sort order.
    = ASC
    -- | Descending sort order.
    | DESC
    deriving (Eq, Show)

-- | `asc` is @Just ASC@, for convenience when using `indexField`.
asc :: Maybe SortOrder
asc = Just ASC

-- | `desc` is @Just DESC@, for convenience when using `indexField`.
desc :: Maybe SortOrder
desc = Just DESC

-- | Enumerates different orders for where @NULL@ values should appear.
data NullsOrder
    -- | @NULL@ values should appear as the first elements in the index.
    = NullsFirst
    -- | @NULL@ values should appear as the last elements in the index.
    | NullsLast
    deriving (Eq, Show)

-- | `nullsFirst` is @Just NullsFirst@, for convenience when using `indexField`.
nullsFirst :: Maybe NullsOrder
nullsFirst = Just NullsFirst

-- | `nullsLast` is @Just NullsLast@, for convenience when using `indexField`.
nullsLast :: Maybe NullsOrder
nullsLast = Just NullsLast

-- | Represents a field for inclusion in a search index. The phantom type
-- parameter @rec@ is used to ensure that the fields belong to the table
-- for which the index is created.
data IndexField rec
    = IdxField { idxFieldName :: !T.Text, idxSql :: !T.Text }

-- | `indexField` @sortOrder nullsOrder entityField@ constructs an `IndexField`
-- for @entityField@ where the optional @sortOrder@ and @nullsOrder@ determine
-- the order of values in the index.
indexField
    :: forall rec typ. PersistEntity rec
    => Maybe SortOrder
    -> Maybe NullsOrder
    -> EntityField rec typ
    -> IndexField rec
indexField mSortOrder mNullOrder entityField = IdxField{
        idxFieldName = fieldName,
        idxSql = T.concat ["\"", fieldName, "\"", sortOrder, nullOrder]
    }
    where
        sortOrder = case mSortOrder of
            Nothing -> T.empty
            Just ASC -> " ASC"
            Just DESC -> " DESC"
        nullOrder = case mNullOrder of
            Nothing -> T.empty
            Just NullsFirst -> " NULLS FIRST"
            Just NullsLast -> " NULLS LAST"
        fieldName =
            unFieldNameDB . fieldDB $
            persistFieldDef entityField


-- | `createIndex` @fields sortOrder@ creates a `Migration` which creates a
-- search index for @fields@ if it does not yet exist.
createIndex
    :: forall rec. PersistEntity rec
    => [IndexField rec]
    -> Migration
createIndex entityFields = addMigration False $ T.concat
    [ "CREATE INDEX IF NOT EXISTS "
    , indexName, " ON \""
    , tableName, "\" (", T.intercalate ", " fieldSql, ") "
    ]
    where
        fieldNames = map idxFieldName entityFields
        fieldSql = map idxSql entityFields
        tableName =
            unEntityNameDB . getEntityDBName $
            entityDef (Proxy :: Proxy rec)
        indexName = T.concat
            [ tableName, "_", T.intercalate "_" fieldNames, "_idx" ]

--------------------------------------------------------------------------------
