{-# LANGUAGE GeneralizedNewtypeDeriving #-}

-- | Implements the `ApiHandler` monad, which represents computations that
-- handle API endpoints.
module Home.API.Server.Handler (
    ApiHandler(..),
    fromApiHandler
) where

--------------------------------------------------------------------------------

import Control.Monad.Except ( MonadError )
import Control.Monad.Reader

import Servant.Server

import Home.API.Server.Context

--------------------------------------------------------------------------------

-- | This type adds read-only state for `ApiContext` on top of Servant's
-- ordinary `Handler` type.
newtype ApiHandler a
    = MkApiHandler { runApiHandler :: ReaderT ApiContext Handler a }
    deriving newtype ( Functor, Applicative, Monad, MonadIO
                     , MonadError ServerError
                     , MonadReader ApiContext
                     )

-- | `fromApiHandler` @ctx handler@ is a monad morphism from `ApiHandler` to
-- `Handler`. That is, given a @ctx@, it allows an `ApiHandler` computation
-- to be run in a `Handler` computation.
fromApiHandler :: ApiContext -> ApiHandler a -> Handler a
fromApiHandler ctx = flip runReaderT ctx . runApiHandler

--------------------------------------------------------------------------------
