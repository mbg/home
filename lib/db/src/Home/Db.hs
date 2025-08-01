--------------------------------------------------------------------------------

module Home.Db (
    DbPool,
    DbQuery,
    withDatabase,
    withPool,
    runMigration
) where

--------------------------------------------------------------------------------

import Control.Monad.Logger
import Control.Monad.Reader
import Control.Monad.IO.Unlift

import Data.Pool ( Pool )
import Data.Text ( Text )

import Database.Persist.Sql
import Database.Persist.Postgresql

import Home.Db.Schema ( migrateAll )
import Home.Db.Config

--------------------------------------------------------------------------------

-- | An alias for @`Pool` `SqlBackend`@ to avoid having to import "Data.Pool"
-- everywhere.
type DbPool = Pool SqlBackend

-- | An alias for @`ReaderT` `SqlBackend` `IO`@ to avoid having to write that
-- whenever a query computation is accepted somewhere.
type DbQuery = ReaderT SqlBackend IO

-- | `withDatabase` @config cont@ is a utility function which initialises a
-- database connection pool using the values from @config@ and then runs @cont@
-- with the pool as argument. The connection pool is deallocated after @cont@
-- returns.
withDatabase
    :: (MonadLoggerIO m, MonadUnliftIO m)
    => DbConfig Text -> (Pool SqlBackend -> m a) -> m a
withDatabase cfg =
    withPostgresqlPool (toConnStr cfg) poolSize
    where poolSize = 10 -- dbConfigPoolSize cfg

-------------------------------------------------------------------------------

-- | `withPool` @pool query@ runs @query@ using a connection from @pool@.
-- This is just `runSqlPool` with its arguments flipped.
withPool :: DbPool -> DbQuery a -> IO a
withPool = flip runSqlPool

--------------------------------------------------------------------------------
