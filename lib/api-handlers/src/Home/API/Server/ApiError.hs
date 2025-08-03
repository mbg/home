-- | Implements the `ApiError` type which is comparable to `ServerError`, except
-- it is intended for JSON responses. Servant `ErrorFormatters` that format errors
-- as JSON are also exported as `jsonErrorFormatters`.
module Home.API.Server.ApiError (
    ApiError(..),
    apiError400,
    apiError404,
    jsonErrorFormatters
) where

--------------------------------------------------------------------------------

import Data.Aeson
import Data.Text ( Text, pack )

import Servant.Server

--------------------------------------------------------------------------------

-- | Represents API errors that will be returned as JSON.
data ApiError = MkApiError {
    -- | The HTTP status code.
    apiErrorCode :: Int,
    -- | The HTTP status code description.
    apiErrorStatus :: Text,
    -- | The error message.
    apiErrorMessage :: Text
} deriving (Eq, Show)

instance ToJSON ApiError where
    toJSON :: ApiError -> Value
    toJSON MkApiError{..} =
        object [ "code" .= apiErrorCode
               , "status" .= apiErrorStatus
               , "message" .= apiErrorMessage
               ]

instance FromJSON ApiError where
    parseJSON = withObject "ApiError" $ \obj ->
        MkApiError <$> obj .: "code"
                   <*> obj .: "status"
                   <*> obj .: "message"

-- `apiError400` @message@ constructs a HTTP 400 error with @message@.
apiError400 :: Text -> ApiError
apiError400 ex = MkApiError{
    apiErrorCode = 400,
    apiErrorStatus = "Bad Request",
    apiErrorMessage = ex
}

-- | `apiError404` is a generic HTTP 404 error.
apiError404 :: ApiError
apiError404 = MkApiError{
    apiErrorCode = 404,
    apiErrorStatus = "Not Found",
    apiErrorMessage = "The requested file was not found."
}

-- | `jsonFormatter` @error@ formats the body of @error@ as JSON.
jsonFormatter :: ErrorFormatter
jsonFormatter _ _ ex = err400{
    errBody = encode $ apiError400 (pack ex)
}

-- | `jsonErrorFormatters` is a Servant `ErrorFormatters` value where all
-- errors are formatted as JSON.
jsonErrorFormatters :: ErrorFormatters
jsonErrorFormatters = defaultErrorFormatters{
    bodyParserErrorFormatter = jsonFormatter,
    urlParseErrorFormatter = jsonFormatter,
    headerParseErrorFormatter = jsonFormatter,
    notFoundErrorFormatter = \_req -> err404{
        errBody = encode apiError404
    }
}

--------------------------------------------------------------------------------
