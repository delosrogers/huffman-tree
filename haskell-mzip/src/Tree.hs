module Tree (
    makeTree,
) where

import Data.Char as Char
import Data.HashMap.Strict as HashMap
import qualified Data.HashSet as HashSet
import Data.List as List
import Data.Maybe as Maybe
import Types (Huff (Node), childChars, children, count)

makeTree :: String -> Types.Huff
makeTree input_str =
    structureTree $ buildBaseTree (Node Nothing 0 [] HashSet.empty) $ HashMap.toList $ makeCountMap $ input_str

structureTree :: Types.Huff -> Types.Huff
structureTree huff
    | List.length (children huff) == 2 = huff
    | otherwise =
        let children_sorted = List.sortBy (\huff1 huff2 -> compare (count huff1) (count huff2)) (children huff)
            smallests = take 2 children_sorted
            smallest = head smallests
            second_smallest = smallests !! 1
         in structureTree
                ( huff
                    { children =
                        Node
                            Nothing
                            (count smallest + count second_smallest)
                            [smallest, second_smallest]
                            (HashSet.union (childChars smallest) (childChars second_smallest)) :
                        List.drop 2 children_sorted
                    }
                )

buildBaseTree :: Huff -> [(Char, Int)] -> Huff
buildBaseTree huff [] = huff
buildBaseTree huff (first : list) =
    buildBaseTree
        huff
            { children = Node (Just (fst first)) (snd first) [] (HashSet.singleton (fst first)) : children huff
            , count = count huff + snd first
            , childChars = (HashSet.insert (fst first) (childChars huff))
            }
        list

-- prependToMaybeList :: a -> Maybe [a] -> Maybe [a]
-- prependToMaybeList item Nothing = Just [item]
-- prependToMaybeList item (Just secondList) = Just (item : secondList)

makeCountMap :: String -> HashMap Char Int
makeCountMap =
    List.foldl insertToHashMap HashMap.empty

insertToHashMap :: HashMap Char Int -> Char -> HashMap Char Int
insertToHashMap map char =
    insertToHashMapPatternMatch (HashMap.lookup char map) char map

insertToHashMapPatternMatch :: Maybe Int -> Char -> HashMap Char Int -> HashMap Char Int
insertToHashMapPatternMatch (Just count) char map = HashMap.insert char (count + 1) map
insertToHashMapPatternMatch Nothing char map = HashMap.insert char 1 map