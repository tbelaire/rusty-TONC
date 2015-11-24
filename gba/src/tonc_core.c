
typedef unsigned char  u8,  byte, uchar, echar;
typedef unsigned short u16, hword, ushort, eshort;
typedef unsigned int   u32, word, uint, eint;
typedef unsigned long long u64;

typedef signed char  s8;
typedef signed short s16;
typedef signed int   s32;
typedef signed long long s64;

typedef volatile u8  vu8;
typedef volatile u16 vu16;
typedef volatile u32 vu32;
typedef volatile u64 vu64;

typedef volatile s8  vs8;
typedef volatile s16 vs16;
typedef volatile s32 vs32;
typedef volatile s64 vs64;

typedef const u8  cu8;
typedef const u16 cu16;
typedef const u32 cu32;
typedef const u64 cu64;

typedef const s8  cs8;
typedef const s16 cs16;
typedef const s32 cs32;
typedef const s64 cs64;

#ifndef NULL
#define NULL (void*)0
#endif



//! VRAM-safe cpy.
/*!	This version mimics memcpy in functionality, with 
	the benefit of working for VRAM as well. It is also 
	slightly faster than the original memcpy, but faster 
	implementations can be made.
	\param dst	Destination pointer.
	\param src	Source pointer.
	\param size	Fill-length in bytes.
	\return		\a dst.
	\note	The pointers and size need not be word-aligned.
*/
void *tonccpy(void *dst, const void *src, uint size)
{
	if(size == 0 || dst==NULL || src==NULL)
		return dst;

	uint count;
	u16 *dst16;		// hword destination
	u8  *src8;		// byte source

	// Ideal case: copy by 4x words. Leaves tail for later.
	if( ((u32)src|(u32)dst)%4==0 && size>=4)
	{
		u32 *src32= (u32*)src, *dst32= (u32*)dst;

		count= size/4;
		uint tmp= count&3;
		count /= 4;

		// Duff, bitch!
		switch(tmp) {
			do {	*dst32++ = *src32++;
		case 3:		*dst32++ = *src32++;
		case 2:		*dst32++ = *src32++;
		case 1:		*dst32++ = *src32++;
		case 0:		; }	while(count--);
		}

		// Check for tail
		size &= 3;
		if(size == 0)
			return dst;

		src8= (u8*)src32;
		dst16= (u16*)dst32;
	}
	else		// Unaligned.
	{
		uint dstOfs= (u32)dst&1;
		src8= (u8*)src;
		dst16= (u16*)(dst-dstOfs);

		// Head: 1 byte.
		if(dstOfs != 0)
		{
			*dst16= (*dst16 & 0xFF) | *src8++<<8;
			dst16++;
			if(--size==0)
				return dst;
		}
	}

	// Unaligned main: copy by 2x byte.
	count= size/2;
	while(count--)
	{
		*dst16++ = src8[0] | src8[1]<<8;
		src8 += 2;
	}

	// Tail: 1 byte.
	if(size&1)
		*dst16= (*dst16 &~ 0xFF) | *src8;

	return dst;
}

