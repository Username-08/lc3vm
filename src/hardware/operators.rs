pub enum Oprtr {
    OPbr,   /* branch */
    OPadd,  /* add  */
    OPld,   /* load */
    OPst,   /* store */
    OPjsr,  /* jump register */
    OPand,  /* bitwise and */
    OPldr,  /* load register */
    OPstr,  /* store register */
    OPrti,  /* unused */
    OPnot,  /* bitwise not */
    OPldi,  /* load indirect */
    OPsti,  /* store indirect */
    OPjmp,  /* jump */
    OPres,  /* reserved (unused) */
    OPlea,  /* load effective address */
    OPtrap, /* execute trap */
}
