-- | Implements validation helpers for API handlers.
module Home.API.Server.Validation (
    validateNonEmpty
) where

--------------------------------------------------------------------------------

import Control.Monad

import Data.Char ( isSpace )
import Data.Text qualified as T

import Home.API.Server.ApiError

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

--------------------------------------------------------------------------------
