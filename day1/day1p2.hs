import Data.Char (digitToInt, isDigit)
import Data.Maybe qualified

main :: IO ()
main = do
  document <- readFile "day1/day1_input.txt"
  let documentLines = lines $ convertWordsToDigits document
  print (getCalibrationSum documentLines)

wordDigit :: [(String, String)]
wordDigit =
  [ ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9")
  ]

-- convert a spelled-out number to a digit using the dictionar
wordToDigit :: String -> String
wordToDigit input = Data.Maybe.fromMaybe input (lookup input wordDigit)

-- idea of algorithm to replace... look for ones, then look for threes, then look for fours and so on
-- replace each of those occurences with the digit using wordToDigit
convertWordsToDigits :: String -> String
convertWordsToDigits [] = []
convertWordsToDigits (x : xs)
  | isDigit x = x : convertWordsToDigits xs
  | word3 `elem` digits = wordToDigit word3 ++ convertWordsToDigits xs
  | word4 `elem` digits = wordToDigit word4 ++ convertWordsToDigits xs
  | word5 `elem` digits = wordToDigit word5 ++ convertWordsToDigits xs
  | otherwise = x : convertWordsToDigits xs
  where
    -- split based on first 3, 4, 5 letters
    digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
    word3 = take 3 (x : xs)
    word4 = take 4 (x : xs)
    word5 = take 5 (x : xs)

getCalibrationVal :: String -> Int
getCalibrationVal xs = digitToInt (head digits) * 10 + digitToInt (last digits)
  where
    digits = filter isDigit xs

getCalibrationSum :: [String] -> Int
getCalibrationSum = sum . map getCalibrationVal
