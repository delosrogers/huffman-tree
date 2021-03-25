module Types (
    Huff (Node),
    value,
    count,
    childChars,
    children,
) where

import qualified Data.HashSet as HashSet

data Huff = Node
    { value :: Maybe Char
    , count :: Int
    , children :: [Huff]
    , childChars :: HashSet.HashSet Char
    }
    deriving (Show)