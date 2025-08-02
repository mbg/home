
-- | Implements the finance API.
module Home.API.Finance (
    FinanceAPI
) where

--------------------------------------------------------------------------------

import Servant

import Home.API.Finance.Merchants

--------------------------------------------------------------------------------

type FinanceAPI
    = "merchants" :> MerchantsAPI

--------------------------------------------------------------------------------
