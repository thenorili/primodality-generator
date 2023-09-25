TODO
- prompt before overwriting files
- use different names for files that have different from/to flags
- explain line 2 better (number of items)
    - is from/to working? i'm getting 1-8 when i pass 1-2
        - it appends it to the end of the existing file, it doesn't overwrite
- Start with a 1 (we use a 2, 3, etc, start with a 1)
- can I just ask the cargo.toml for the version instead of hard-coding?

```
NOTE:
cool nushell one-liner here for making structured data -- splits it into tables, num, den, hz -- might be a cool idea to make structured data (though ofc .scl is structured enough for scala lol)

open primodality-41.scl | lines | where $it =~ "^[1-9][^p]+$" | parse "{numerator}/{denominator}" | into int numerator denominator | insert hz { | x | $x.numerator / $x.denominator * 440 }

open primodality-41.scl | lines | where $it =~ "^[1-9][^p]+$" | each { | rational | if ( $rational =~ ".*/.*" ) { $rational | parse "{numerator}/{denominator}" } else { { numerator:$rational, denominator:1 } } } | flatten | into int numerator denominator | insert hz { | x | $x.numerator / $x.denominator * 440 }

open primodality-41.scl | lines | where $it =~ "^[1-9][^p]*$" | each { | rational | if ( $rational =~ ".*\/.*" ) { $rational | parse "{numerator}/{denominator}" } else { { numerator:$rational, denominator:"1" } | into int numerator denominator | {numerator:($in.numerator * 41 ), denominator: ($in.denominator * 41 )} } } | flatten | into int numerator denominator | | sort-by numerator | uniq | insert hz { | x | $x.numerator / $x.denominator * 440 }

open primodality-41.scl | lines | where $it =~ "^[1-9][^p]*$" | each { | rational | if ( $rational =~ ".*\/.*" ) { $rational | parse "{numerator}/{denominator}" } else { { numerator:$rational, denominator:"1" } } } | flatten | into int numerator denominator | insert hz { | x | $x.numerator / $x.denominator * 440 } | uniq | sort-by hz
