module Lib (
    someFunc,
) where

import Tree (makeTree)

someFunc :: IO ()
someFunc = putStrLn (show (makeTree "helllosigc"))

-- Node
--     { value = Nothing
--     , count = 10
--     , children =
--         [ Node
--             { value = Nothing
--             , count = 6
--             , children =
--                 [ Node
--                     { value = Nothing
--                     , count = 3
--                     , children =
--                         [ Node{value = Just 'c', count = 1, children = [], childChars = "c"}
--                         , Node
--                             { value = Nothing
--                             , count = 2
--                             , children =
--                                 [ Node{value = Just 'e', count = 1, children = [], childChars = "e"}
--                                 , Node{value = Just 's', count = 1, children = [], childChars = "s"}
--                                 ]
--                             , childChars = "es"
--                             }
--                         ]
--                     , childChars = "ces"
--                     }
--                 , Node{value = Just 'l', count = 3, children = [], childChars = "l"}
--                 ]
--             , childChars = "cesl"
--             }
--         , Node
--             { value = Nothing
--             , count = 4
--             , children =
--                 [ Node
--                     { value = Nothing
--                     , count = 2
--                     , children =
--                         [ Node{value = Just 'h', count = 1, children = [], childChars = "h"}
--                         , Node{value = Just 'g', count = 1, children = [], childChars = "g"}
--                         ]
--                     , childChars = "hg"
--                     }
--                 , Node
--                     { value = Nothing
--                     , count = 2
--                     , children =
--                         [ Node{value = Just 'o', count = 1, children = [], childChars = "o"}
--                         , Node{value = Just 'i', count = 1, children = [], childChars = "i"}
--                         ]
--                     , childChars = "oi"
--                     }
--                 ]
--             , childChars = "hgoi"
--             }
--         ]
--     , childChars = "cseghilo"
--     }