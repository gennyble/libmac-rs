#include "All.h"
#include "CharacterHelper.h"

namespace APE
{

str_ansi * CAPECharacterHelper::GetANSIFromUTF8(const str_utf8 * pUTF8)
{
    str_utfn * pUTF16 = GetUTF16FromUTF8(pUTF8);
    str_ansi * pANSI = GetANSIFromUTF16(pUTF16);
    delete [] pUTF16;
    return pANSI;
}

str_ansi * CAPECharacterHelper::GetANSIFromUTF16(const str_utfn * pUTF16)
{
    const int nCharacters = pUTF16 ? static_cast<int>(wcslen(pUTF16)) : 0;
    #ifdef _WIN32
        const int nANSICharacters = (2 * nCharacters);
        str_ansi * pANSI = new str_ansi [static_cast<size_t>(nANSICharacters + 1)];
        const size_t nBufferBytes = (sizeof(pANSI[0]) * static_cast<size_t>(nCharacters + 1));
        memset(pANSI, 0, nBufferBytes);
        if (pUTF16)
            WideCharToMultiByte(CP_ACP, 0, pUTF16, -1, pANSI, nANSICharacters, APE_NULL, APE_NULL);
    #else
        str_utf8 * pANSI = new str_utf8 [nCharacters + 1];
        for (int z = 0; z < nCharacters; z++)
            pANSI[z] = (pUTF16[z] >= 256) ? '?' : (str_utf8) pUTF16[z];
        pANSI[nCharacters] = 0;
    #endif

    return reinterpret_cast<str_ansi *>(pANSI);
}

str_utfn * CAPECharacterHelper::GetUTF16FromANSI(const str_ansi * pANSI)
{
    const int nCharacters = pANSI ? static_cast<int>(strlen(pANSI)) : 0;
    str_utfn * pUTF16 = new str_utfn [static_cast<size_t>(nCharacters + 1)];

    #ifdef _WIN32
        const size_t nBufferBytes = (sizeof(pUTF16[0]) * static_cast<size_t>(nCharacters + 1));
        memset(pUTF16, 0, nBufferBytes);
        if (pANSI)
            MultiByteToWideChar(CP_ACP, 0, pANSI, -1, pUTF16, nCharacters);
    #else
        for (int z = 0; z < nCharacters; z++)
            pUTF16[z] = (str_utfn) ((str_utf8) pANSI[z]);
        pUTF16[nCharacters] = 0;
    #endif

    return pUTF16;
}

str_utfn * CAPECharacterHelper::GetUTF16FromUTF8(const str_utf8 * pUTF8)
{
    // get the length
    int nCharacters = 0; int nIndex = 0;
    while (pUTF8[nIndex] != 0)
    {
        if ((pUTF8[nIndex] & 0x80) == 0)
            nIndex += 1;
        else if ((pUTF8[nIndex] & 0xE0) == 0xE0)
            nIndex += 3;
        else
            nIndex += 2;

        nCharacters += 1;
    }

    // make a UTF-16 string
    str_utfn * pUTF16 = new str_utfn[static_cast<size_t>(nCharacters) + 1];
    str_utfn * pOutput = pUTF16;
    nIndex = 0;
    while (pUTF8[nIndex] != 0)
    {
        if ((pUTF8[nIndex] & 0x80) == 0)
        {
            *pOutput++ = pUTF8[nIndex];
            nIndex += 1;
        }
        else if ((pUTF8[nIndex] & 0xE0) == 0xE0)
        {
            *pOutput++ = static_cast<str_utfn>(((pUTF8[nIndex] & 0x1F) << 12) | ((pUTF8[nIndex + 1] & 0x3F) << 6) | (pUTF8[nIndex + 2] & 0x3F));
            nIndex += 3;
        }
        else
        {
            *pOutput++ = static_cast<str_utfn>(((pUTF8[nIndex] & 0x3F) << 6) | (pUTF8[nIndex + 1] & 0x3F));
            nIndex += 2;
        }
    }
    *pOutput++ = 0; // NULL terminate

    return pUTF16;
}

str_utf8 * CAPECharacterHelper::GetUTF8FromANSI(const str_ansi * pANSI)
{
    str_utfn * pUTF16 = GetUTF16FromANSI(pANSI);
    str_utf8 * pUTF8 = GetUTF8FromUTF16(pUTF16);
    delete [] pUTF16;
    return pUTF8;
}

str_utf8 * CAPECharacterHelper::GetUTF8FromUTF16(const str_utfn * pUTF16)
{
    // get the size(s)
    const int nCharacters = static_cast<int>(wcslen(pUTF16));
    int nUTF8Bytes = 0;
    for (int z = 0; z < nCharacters; z++)
    {
        if (pUTF16[z] < 0x0080) {
            printf("[%x]: 1\n", pUTF16[z]);
            nUTF8Bytes += 1;
        } else if (pUTF16[z] < 0x0800) {
            printf("[%x]: 2\n", pUTF16[z]);
            nUTF8Bytes += 2;
        } else if (pUTF16[z] > 0xD800 && pUTF16[z] < 0xDFFF) {
            // We're a surrogate.
            if (pUTF16[z] < 0xDC00) {
                // We're the high part

                if (z + 1 < nCharacters && (pUTF16[z+1] > 0xDC00 && pUTF16[z+1] < 0xDFFF)) {
                    // Next chracter is the low-part. This is a valid pairing. Skip the
                    // pair as it's factored in here
                    z++;
                    nUTF8Bytes += 4;
                    printf("[%x %x]: 4 (surrogate)\n", pUTF16[z], pUTF16[z+1]);
                } else {
                    // there are no more characters; we're missing the low so
                    // we should emit the replacement character U+FFFD (3 bytes utf8)
                    // OR the next was not the low part and we need the replacement
                    nUTF8Bytes += 3;
                    printf("[%x]: 3 (unpaired surrogate)\n", pUTF16[z]);
                }
            } else {
                // we're the low part which is an error if it's first and
                // we're LE UTF-16
                nUTF8Bytes += 3;
                printf("[%x]: 3 (malformed surrogate)\n", pUTF16[z]);
            }
        } else {
            printf("[%x]: 3\n", pUTF16[z]);
            nUTF8Bytes += 3;
        }
    }

    // allocate a UTF-8 string
    str_utf8 * pUTF8 = new str_utf8 [static_cast<size_t>(nUTF8Bytes) + 1];

    // create the UTF-8 string
    str_utf8 * pOutput = pUTF8;
    for (int z = 0; z < nCharacters; z++)
    {
        if (pUTF16[z] < 0x0080)
        {
            *pOutput++ = static_cast<str_utf8>(pUTF16[z]);
        }
        else if (pUTF16[z] < 0x0800)
        {
            *pOutput++ = static_cast<str_utf8>(0xC0 | (pUTF16[z] >> 6));
            *pOutput++ = static_cast<str_utf8>(0x80 | (pUTF16[z] & 0x3F));
        }
        else if (pUTF16[z] > 0xD800 && pUTF16[z] < 0xDFFF)
        {
            if (pUTF16[z] < 0xDC00) {
                if (z + 1 < nCharacters && (pUTF16[z+1] > 0xDC00 && pUTF16[z+1] < 0xDFFF)) {
                    str_utfn high = pUTF16[z] & 0x3FF;
                    str_utfn low = pUTF16[z + 1] & 0x3FF;
                    uint32 codepoint = low | (high << 10) | 0x10000;
                    //uint32 codepoint = high | (low << 10);
                    printf("high: %x (%x)\nlow: %x (%x)\n", pUTF16[z], high, pUTF16[z+1], low);
                    printf("codepoint: %x\n", codepoint);

                    *pOutput++ = static_cast<str_utf8>(0xF0 | ((codepoint & 0x1C0000) >> 18));
                    *pOutput++ = static_cast<str_utf8>(0x80 | ((codepoint & 0x3F000) >> 12));
                    *pOutput++ = static_cast<str_utf8>(0x80 | ((codepoint & 0xFC0) >> 6));
                    *pOutput++ = static_cast<str_utf8>(0x80 | (codepoint & 0x3F));

                    // jump over the 2nd part of the pair
                    z++;
                } else {
                    *pOutput++ = static_cast<str_utf8>(0xEF);
                    *pOutput++ = static_cast<str_utf8>(0xBF);
                    *pOutput++ = static_cast<str_utf8>(0xBD);
                }
            } else {
                *pOutput++ = static_cast<str_utf8>(0xEF);
                *pOutput++ = static_cast<str_utf8>(0xBF);
                *pOutput++ = static_cast<str_utf8>(0xBD);
            }
        }
        else
        {
            *pOutput++ = static_cast<str_utf8>(0xE0 | (pUTF16[z] >> 12));
            *pOutput++ = static_cast<str_utf8>(0x80 | ((pUTF16[z] >> 6) & 0x3F));
            *pOutput++ = static_cast<str_utf8>(0x80 | (pUTF16[z] & 0x3F));
        }
    }
    *pOutput++ = 0;

    printf("from utf16->utf8:\n\t");
    for (int i = 0; i < nUTF8Bytes; ++i) {
        printf("%x ", pUTF8[i]);
    }
    printf("\n");

    // return the UTF-8 string
    return pUTF8;
}

}
