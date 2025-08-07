-- | Implements validation helpers for API handlers.
module Home.API.Server.Validation (
    validateNonEmpty,
    validateNoEntity
) where

--------------------------------------------------------------------------------

import Control.Monad

import Data.Char ( isSpace )
import Data.Text qualified as T

import Database.Esqueleto.Experimental

import Home.API.Server.ApiError
import Home.Db

--------------------------------------------------------------------------------

-- | `validateNonEmpty` @name val@ validates that @val@ is not made up of only
-- space characters. If it is, then a HTTP 400 error is thrown whose body
-- explains that @name@ must not be empty.
validateNonEmpty
    :: MonadApiError m
    => T.Text
    -> T.Text
    -> m ()
validateNonEmpty name val =
    when (T.all isSpace val) $ throwApiError $ apiError400 $
        name <> " must not be empty."

-- | `validateNoEntity` @msg query@ runs @query@. If @query@ returns any results,
-- then a HTTP 400 error with @msg@ is thrown.
validateNoEntity
    :: (CanRunQuery m, MonadApiError m, SqlSelect a r)
    => T.Text
    -> SqlQuery a
    -> m ()
validateNoEntity msg q = do
    rs <- runQuery $ select q
    when (not $ null rs) $ throwApiError $ apiError400 msg

--------------------------------------------------------------------------------
