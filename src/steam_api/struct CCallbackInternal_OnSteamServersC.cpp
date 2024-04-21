struct CCallbackInternal_OnSteamServersConnected : private CCallbackImpl< sizeof( SteamServersConnected_t ) > {
  CCallbackInternal_OnSteamServersConnected () {
    this->SetGameserverFlag();
    SteamAPI_RegisterCallback( this, SteamServersConnected_t::k_iCallback );
  }
  CCallbackInternal_OnSteamServersConnected ( const CCallbackInternal_OnSteamServersConnected & ) {
    this->SetGameserverFlag();
    SteamAPI_RegisterCallback( this, SteamServersConnected_t::k_iCallback );
  }
  CCallbackInternal_OnSteamServersConnected & operator=( const CCallbackInternal_OnSteamServersConnected & ) { return *this; }

  private:
  virtual void Run( void *pvParam ) {
    CSpaceWarServer *pOuter = reinterpret_cast<CSpaceWarServer*>(
      reinterpret_cast<char*>(this) - ((::size_t)&reinterpret_cast<char const volatile&>((((CSpaceWarServer*)0)->m_steamcallback_OnSteamServersConnected)))
    );

    pOuter->OnSteamServersConnected( reinterpret_cast<SteamServersConnected_t*>( pvParam ) ); 
  }
  m_steamcallback_OnSteamServersConnected;
  void OnSteamServersConnected( SteamServersConnected_t *pParam );