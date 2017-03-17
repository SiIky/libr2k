data Dict
data Type s = Auto s | Hiragana s | Katakana s

genConvert :: Dict -> String -> String
genConvert d s = concat $ map (convertWord d) (words s)

convertWord :: Dict -> String -> String
convertWord d s = concat $ map (convertSyllable d) (syllables d s)

syllables :: Dict -> String -> [String]
syllables _ [] = []
syllables d s@(x:xs)
    | fst cond = (:) (fst $ snd $ cond) (syllables d (snd $ snd $ cond))
    | otherwise = (:) [x] (syllables d xs)
    where
        max = let len = length s in if (>) len 3 then 3 else len
        aux :: Int -> Dict -> String -> (Bool, (String, String))
        aux 0 _ _ = (False, ([], []))
        aux _ _ [] = (False, ([], []))
        aux n d s
            | d `containsKey` (take n s) = (True, (splitAt n s))
            | otherwise = aux ((-) n 1) d s
        cond = aux max d s

toKana :: Dict -> Type String -> String
toKana d (Auto s) = genConvert d s
toKana d (Hiragana s) = genConvert d $ toLowercse s
toKana d (Katakana s) = genConvert d $ toUppercase s

containsKey :: Dict -> String -> Bool
containsKey = undefined

convertSyllable :: Dict -> String -> String
convertSyllable = undefined

toLowercse :: String -> String
toLowercse = undefined

toUppercase :: String -> String
toUppercase = undefined
