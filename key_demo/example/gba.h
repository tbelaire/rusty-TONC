
//{{BLOCK(gba)

//======================================================================
//
//	gba, 240x160@8, 
//	+ palette 256 entries, not compressed
//	+ 600 tiles not compressed
//	Total size: 512 + 38400 = 38912
//
//	Time-stamp: 2015-11-22, 19:09:52
//	Exported by Cearn's GBA Image Transmogrifier, v0.8.12
//	( http://www.coranac.com/projects/#grit )
//
//======================================================================

#ifndef GRIT_GBA_H
#define GRIT_GBA_H

#define gbaTilesLen 38400
extern const unsigned int gbaTiles[9600];

#define gbaPalLen 512
extern const unsigned short gbaPal[256];

#endif // GRIT_GBA_H

//}}BLOCK(gba)
