{-# LANGUAGE GeneralizedNewtypeDeriving #-}

-- | Implements the `ApiHandler` monad, which represents computations that
-- handle API endpoints.
module Home.API.Server.Handler (
    ApiHandler(..),
    fromApiHandler,
    selectOneOr404,
    -- * Re-exports
    module Servant,
    module Home.Db.Types,
    module Home.API.Server.ApiError,
    module Home.API.Server.Validation,
    CanRunQuery(..)
) where

--------------------------------------------------------------------------------

import Control.Monad.Reader

import Data.Aeson ( encode )
import Data.Text qualified as T

import Servant

import Database.Esqueleto.Experimental

import Home.Db
import Home.Db.Types
import Home.API.Server.ApiError
import Home.API.Server.Context
import Home.API.Server.Validation

--------------------------------------------------------------------------------

-- | This type adds read-only state for `ApiContext` on top of Servant's
-- ordinary `Handler` type.
newtype ApiHandler a
    = MkApiHandler { runApiHandler :: ReaderT ApiContext Handler a }
    deriving newtype ( Functor, Applicative, Monad, MonadIO
                     , MonadReader ApiContext
                     )

instance MonadApiError ApiHandler where
    throwApiError :: ApiError -> ApiHandler r
    throwApiError err@MkApiError{..} = MkApiHandler $ throwError $ ServerError{
        errHTTPCode = apiErrorCode,
        errReasonPhrase = T.unpack apiErrorStatus,
        errBody = encode err,
        errHeaders = []
    }

-- | `fromApiHandler` @ctx handler@ is a monad morphism from `ApiHandler` to
-- `Handler`. That is, given a @ctx@, it allows an `ApiHandler` computation
-- to be run in a `Handler` computation.
fromApiHandler :: ApiContext -> ApiHandler a -> Handler a
fromApiHandler ctx = flip runReaderT ctx . runApiHandler

-- | `selectOneOr404` @query@ performs @query@ which is expected to return
-- a single result. If there is none, a HTTP 404 error is raised as a
-- `ServerError`.
selectOneOr404
    :: (CanRunQuery m, MonadApiError m, SqlSelect a r)
    => SqlQuery a -> m r
selectOneOr404 = selectOneOr (throwApiError apiError404)

instance CanRunQuery ApiHandler where
    runQuery :: DbQuery a -> ApiHandler a
    runQuery q = asks apiContextDbPool >>= liftIO . runSqlPool q

--------------------------------------------------------------------------------
