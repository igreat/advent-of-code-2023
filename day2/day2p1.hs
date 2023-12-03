import Data.Char (isDigit)

main :: IO ()
main = do
  document <- readFile "day2/day2_input.txt"
  let documentLines = lines document
  print (sum [id | (id, r, g, b) <- countColorsAll documentLines, r <= 12 && g <= 13 && b <= 14])

countColors :: String -> (Int, Int, Int) -> (Int, Int, Int) -- (r, g, b)
countColors str (r, g, b)
  | null rest = (r, g, b)
  | color == "red" = countColors (tail rest) (max r num, g, b)
  | color == "green" = countColors (tail rest) (r, max g num, b)
  | color == "blue" = countColors (tail rest) (r, g, max b num)
  | otherwise = (r, g, b)
  where
    (num_str, other) = splitAtCondition (not . isDigit) str
    num = read num_str :: Int
    (color, rest) = splitAtCondition (\x -> x == ',' || x == ';') other

countColorsAll :: [String] -> [(Int, Int, Int, Int)] -- (id, r, g, b)
-- per line, it actually starts with Game {id}: followed by the rest which would be passed to countColors
countColorsAll [] = []
countColorsAll (x : xs) = (id, r, g, b) : countColorsAll xs
  where
    (_, other) = splitAtCondition (== ' ') x
    (id_str, rest) = splitAtCondition (== ':') other
    id = read id_str :: Int
    (r, g, b) = countColors (tail rest ++ ",  ") (0, 0, 0)

splitAtCondition :: (Char -> Bool) -> String -> (String, String)
splitAtCondition _ [] = ([], [])
splitAtCondition condition (x : xs)
  | condition x = ([], xs)
  | otherwise =
      let (before, after) = splitAtCondition condition xs
       in (x : before, after)
