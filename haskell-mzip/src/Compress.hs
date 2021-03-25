module Compress (
    ) where

-- compress,

import qualified Data.ByteString.Builder as B
import qualified Data.ByteString.Lazy as L
import qualified Data.HashSet as HashSet
import qualified Data.List as List
import qualified Data.Maybe as Maybe
import Data.Word as Word
import Types (Huff, childChars, children, value)

compress :: String -> Huff -> L.ByteString
compress input huff =
    B.toLazyByteString $ List.foldl foldBitsIntoBuilder (B.word8 0) (List.concatMap (\char -> getPathForChar char huff []) input)

foldBitsIntoBuilder :: (Int, Word.Word8, B.Builder) -> Int -> (Int, Int, B.Builder)
foldBitsIntoBuilder (currentBit, currentByte, builder) direction
    | currentBit == 7 =
        ( 0
        , 0
        , builder ++ B.word8 $ (currentByte + (direction ^ 7))
        )
    | otherwise =
        ( currentBit + 1
        , currentByte + (direction ^ 7)
        , builder
        )

getPathForChar :: Char -> Huff -> [Int] -> [Int]
getPathForChar char huff path
    | Maybe.isJust (value huff) = List.reverse path
    | HashSet.member char (childChars $ head $ children $ huff) =
        getPathForChar char (head $ children $ huff) (0 : path)
    | otherwise =
        getPathForChar char (children huff !! 1) (1 : path)
