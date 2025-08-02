-- | Implements the handlers for the finance API.
module Home.API.Server.Finance (
    financeHandlers
) where

--------------------------------------------------------------------------------

import Home.API.Finance
import Home.API.Server.Handler
import Home.API.Server.Finance.Merchants

--------------------------------------------------------------------------------

financeHandlers :: ServerT FinanceAPI ApiHandler
financeHandlers = merchantHandlers

--------------------------------------------------------------------------------
