--------------------------------------------------------------------------------

module Home.Db (
    DbPool,
    DbQuery,
    waitForDb,
    withDatabase,
    withPool,
    runMigration,
    CanRunQuery(..),
    selectOneOr
) where

--------------------------------------------------------------------------------

import Control.Monad
import Control.Monad.IO.Unlift
import Control.Monad.Logger
import Control.Monad.Reader
import Control.Retry

import Data.Pool ( Pool )
import Data.Text ( Text )
import Data.Time.Units

import Database.Esqueleto.Experimental
import Database.Persist.Postgresql

import Network.Wait.PostgreSQL

import Home.Db.Config

--------------------------------------------------------------------------------

-- | An alias for @`Pool` `SqlBackend`@ to avoid having to import "Data.Pool"
-- everywhere.
type DbPool = Pool SqlBackend

-- | An alias for @`ReaderT` `SqlBackend` `IO`@ to avoid having to write that
-- whenever a query computation is accepted somewhere.
type DbQuery = ReaderT SqlBackend IO

-- | `waitForDb` @config@ waits for the server identified by @config@ to
-- become available and accept connections.
waitForDb :: DbConfig Text -> IO ()
waitForDb cfg = do
    let strategy =
            exponentialBackoff (fromInteger $ toMicroseconds @Second 5) <>
            limitRetries 5

    void $ waitPostgreSqlVerbose putStrLn strategy (toConnStr cfg)

-- | `withDatabase` @config cont@ is a utility function which initialises a
-- database connection pool using the values from @config@ and then runs @cont@
-- with the pool as argument. The connection pool is deallocated after @cont@
-- returns.
withDatabase
    :: (MonadLoggerIO m, MonadUnliftIO m)
    => DbConfig Text -> (DbPool -> m a) -> m a
withDatabase cfg =
    withPostgresqlPool (toConnStr cfg) poolSize
    where poolSize = 10 -- dbConfigPoolSize cfg

-------------------------------------------------------------------------------

-- | `withPool` @pool query@ runs @query@ using a connection from @pool@.
-- This is just `runSqlPool` with its arguments flipped.
withPool :: DbPool -> DbQuery a -> IO a
withPool = flip runSqlPool

-------------------------------------------------------------------------------

-- | A class of types which represent contexts in which a `DbQuery` can be
-- executed from.
class Monad m => CanRunQuery m where
    -- | `runQuery` @query@ executes @query@ using a database connection that
    -- is available in a given context.
    runQuery :: DbQuery a -> m a

-- | `selectOneOr` @default query@ performs @query@ which is expected to
-- return a single result. If there is none, then @default@ is executed and
-- its result is returned.
selectOneOr :: CanRunQuery m => SqlSelect a r => m r -> SqlQuery a -> m r
selectOneOr def q = runQuery (selectOne q) >>= maybe def pure

--------------------------------------------------------------------------------
