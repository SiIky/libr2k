data Dict
data Type s = Auto s
            | Hira s
            | Kata s

toKana :: Dict -> Type String -> String
toKana d t = genConvert d $ unwrapType t

genConvert :: Dict -> String -> String
genConvert d s = concatMap (convertWord d) (words s)

convertWord :: Dict -> String -> String
convertWord d s = concatMap (convertSyllable d) (syllables d s)

syllables :: Dict -> String -> [String]
syllables _ [] = []
syllables d s@(x:xs)
    | fst cond  = (:) (fst $ snd $ cond) (syllables d (snd $ snd $ cond))
    | otherwise = (:) [x]                (syllables d xs)
    where
        cond = aux max d s
        max = min 3 $ length s
        aux :: Int -> Dict -> String -> (Bool, (String, String))
        aux 0 _ _  = (False, ([], []))
        aux _ _ [] = (False, ([], []))
        aux n d s
            | d `containsKey` (take n s) = (True, (splitAt n s))
            | otherwise = aux (pred n) d s

unwrapType :: Type String -> String
unwrapType (Auto s) = s
unwrapType (Hira s) = toLowercase s
unwrapType (Kata s) = toUppercase s

containsKey :: Dict -> String -> Bool
containsKey = undefined

convertSyllable :: Dict -> String -> String
convertSyllable = undefined

toLowercase :: String -> String
toLowercase = undefined

toUppercase :: String -> String
toUppercase = undefined
